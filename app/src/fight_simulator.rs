use anyhow::Error;
use app_core::models::*;
use std::{cmp::max, collections::{HashMap, HashSet}, ops::Deref};

use chrono::{DateTime, TimeDelta, Utc};
use data::json::{models::{Class, RawSkill, SkillGrade}, CLASS_MAP, SKILL_MAP};
use log::{debug, info};
use rand::{seq::SliceRandom, Rng};
use serde::Serialize;

const HIT_OPTIONS: [HitOption; 3] = [HitOption::BackAttack, HitOption::FlankAttack, HitOption::FrontalAttack];

pub fn random_hit_option() -> HitOption {
    let random_index = rand::thread_rng().gen_range(0..HIT_OPTIONS.len());
    HIT_OPTIONS[random_index]
}

#[derive(Debug)]
pub struct AttackResult<'a> {
    pub skill: &'a Skill,
    pub has_fight_ended: bool,
    pub hit_flag: HitFlag,
    pub hit_option: HitOption,
    pub damage: i64
}

#[derive(Debug)]
pub struct PlayerWithTemplate {
    pub entity: Player,
    pub template: PlayerTemplate
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerTemplate {
    skills: Vec<Skill>,
    crit_rate: f32,
    min_dmg: i64,
    max_dmg: i64
}

#[derive(Debug)]
pub struct EstherWithTemplate {
    pub entity: Esther,
    pub template: EstherTemplate
}

#[derive(Debug, Clone, Serialize)]
pub struct EstherTemplate {
    pub name: String,
    pub icon: String,
    pub min_dmg: i64,
    pub max_dmg: i64
}

#[derive(Debug, Default)]
pub struct FightSimulator<'a> {
    registered_ids: HashSet<EntityId>,
    class_color_map: HashMap<ClassId, String>,
    skills_by_class: HashMap<SkillId, Vec<&'a RawSkill<'a>>>,
    dps_classes: Vec<&'a Class<'a>>,
    sup_classes: Vec<&'a Class<'a>>,
    registered_dps_class_ids: HashSet<ClassId>,
    players: Vec<PlayerWithTemplate>,
    esthers: Vec<EstherWithTemplate>,
    esther_gauge: f32,
    updated_esther_gauge_on: Option<DateTime<Utc>>,
    last_used_esther_on: Option<DateTime<Utc>>,
    boss: Option<Boss>,
    has_fight_ended: bool,
    current_time: DateTime<Utc>,
    fight_started_on: Option<DateTime<Utc>>,
    duration: TimeDelta,
    stats: EncounterStats
}

impl<'a> FightSimulator<'a> {
    pub fn new() -> Self {
        let dps_classes = CLASS_MAP.values()
            .filter(|class| !class.is_support && !class.is_generic)
            .collect();

        let sup_classes = CLASS_MAP.values()
            .filter(|class| class.is_support)
            .collect();

        let mut skills_by_class = HashMap::new();

        for skill in SKILL_MAP.values() {

            if skill.grade != SkillGrade::Normal {
                continue;
            }

            let name = skill.name
                .filter(|name| *name != "Weapon Attack"
                    && *name != "Hand Attack"
                    && *name !=  "Stand Up");

            match (skill.class_id, name, skill.icon) {
                (Some(class_id), Some(_), Some(_)) => {
                    let entry: &mut Vec<&'a RawSkill> = skills_by_class.entry(class_id).or_default();
                    entry.push(skill);
                },
                _ => {
                    continue;
                }
            }
        }

        Self {
            skills_by_class,
            dps_classes,
            sup_classes,
            esther_gauge: 2.0,
            ..Default::default()
        }
    }

    pub fn create_8_players(&mut self, min_dmg: i64, max_dmg: i64) -> Result<(), Error> {
        let now = Utc::now();
        let mut players =  vec![];

        for index in 1..9 {

            let id = self.get_random_unique_entity_id();
            let name = self.get_random_nickname(10);

            let class = if index % 4 == 0 {
                self.get_random_sup_class()
            }
            else {
                Self::get_random_dps_class(&self.dps_classes, &mut self.registered_dps_class_ids)
            };

            let class_id = class.id;
            let mut class_skills = self.skills_by_class.get(&class_id).cloned().unwrap_or_default();
            let mut template =  PlayerTemplate {
                skills: Self::get_random_skills_for_class(&mut class_skills),
                crit_rate: 0.1,
                min_dmg: 10_000,
                max_dmg: 100_000,
            };

            if template.skills.is_empty() {
                return Err(anyhow::format_err!("Could not find skills for class: {}", class.name.to_string()))?;
            }

            if !class.is_support {
                template.crit_rate = Self::get_random_f32_value(0.5, 1.0);
                template.min_dmg = min_dmg;
                template.max_dmg = max_dmg;
            };

            let player = Player {
                id,
                name,
                class_color: self.class_color_map.get(&class.id).cloned().unwrap_or_else(|| "#FFFFFF".to_string()),
                class_name: class.name.to_string(),
                class_id: class.id,
                stats: PlayerStats::default(),
                death_log: DeathLog::default(),
                created_on: now,
            };

            let player_with_template = PlayerWithTemplate {
                entity: player,
                template
            };

            players.push(player_with_template);
        }

        self.players = players;

        Ok(())
    }

    pub fn set_random_player_dead(&mut self) {
        let now = Utc::now();
        let index = Self::get_random_index(self.players.len());

        let player = self.players.get_mut(index).unwrap();
        let death_log = &mut player.entity.death_log;
        death_log.is_dead = true;
        death_log.recorded_on = Some(now);
        death_log.duration = Some(now - now);
    }

    pub fn configure_esther(&mut self, template: EstherTemplate) {
        let esther = Esther {
            id: self.get_random_unique_entity_id(),
            name: template.name.clone(),
            icon: template.icon.clone(),
            stats: EstherStats::default(),
        };

        let esther = EstherWithTemplate {
            entity: esther,
            template
        };

        self.esthers.push(esther);
    }

    pub fn update_time(&mut self) {
        let now = Utc::now();
        self.current_time = now;
    }

    pub fn update_esther_gauge(&mut self) {
        let now = self.current_time;

        // 1:20 - 80 sec - 1bar
        // start of fight 2 bars
        if let Some(updated_esther_gauge_on) = self.updated_esther_gauge_on {
            let diff = now - updated_esther_gauge_on;
            let seconds_fraction = diff.num_milliseconds() as f32 / 1000.0;
            self.esther_gauge += 0.0125 * seconds_fraction;
            self.updated_esther_gauge_on.replace(now);

            return;
        }

        self.esther_gauge += 0.0125;
        self.updated_esther_gauge_on.get_or_insert(now);
    }

    pub fn try_use_esther(&mut self) {
        
        if self.esther_gauge < 3.0 {
            return;
        }

        let now = self.current_time;
        self.last_used_esther_on.get_or_insert(now);
        self.esther_gauge -= 3.0;

        let esther = Self::get_random_esther(&mut self.esthers);
        let esther_damage = Self::get_random_value(esther.template.min_dmg, esther.template.max_dmg);

        self.stats.total_esther_damage += esther_damage;
        let stats = &mut esther.entity.stats;
        stats.total_damage += esther_damage;
        stats.damage_percentage  = stats.total_damage as f32 / self.stats.total_esther_damage as f32;
    }

    pub fn create_player(&mut self) {

    }

    pub fn has_ended(&self) -> bool {
        self.has_fight_ended
    }

    pub fn update_duration(&mut self) {
        let now = self.current_time;
        let fight_started_on = self.fight_started_on.get_or_insert_with(|| now);
        
        self.duration = now - *fight_started_on;
    }

    pub fn perform_attacks_and_update_stats(&mut self) -> Result<(), Error> {
        
        let duration_seconds = self.duration.num_seconds();
        let now = self.current_time;

        let boss = self.boss.as_mut().ok_or_else(|| anyhow::anyhow!("Boss unset"))?;
        let mut players_who_performed_attack = HashSet::new();
        let (start, end) = Self::generate_random_range(self.players.len() as u32);
        let players_count = self.players.len();

        for _ in start..end {
            let mut player_index = Self::get_random_index(players_count);
            let mut player = &mut self.players[player_index];

            while players_who_performed_attack.contains(&player_index) || player.entity.death_log.is_dead {
                player_index = Self::get_random_index(players_count);
                player = &mut self.players[player_index]
            }

            players_who_performed_attack.insert(player_index);
            
            let current_boss_hp = boss.stats.hp;
            let attack_result = Self::perform_attack(current_boss_hp, &player.template);
            self.stats.total_player_damage += attack_result.damage;
            self.stats.top_damage = max(attack_result.damage, self.stats.top_damage);

            if attack_result.has_fight_ended {
                self.has_fight_ended = true;
                let stats = &mut boss.stats;
                Self::update_boss(stats, &attack_result, now);
                break;
            }

            Self::update_player(
                &mut player.entity,
                &attack_result,
                self.stats.total_player_damage,
                now);
    
            let stats = &mut boss.stats;
            Self::update_boss(stats, &attack_result, now);
        }

        let boss_damage = Self::perform_boss_attack(boss);
        self.stats.total_taken_damage += boss_damage;

        let player = Self::get_random_alive_player(&mut self.players);
        Self::update_damage_taken(player, boss_damage, self.stats.total_taken_damage);

        self.stats.party_dps = Dps::new(self.stats.total_player_damage, duration_seconds);
        let time_to_kill = boss.stats.hp / self.stats.party_dps.raw;
        self.stats.time_to_kill_mmss = Self::seconds_to_mm_ss(time_to_kill);

        self.recalculate_all_stats(now, duration_seconds);

        Ok(())
    }

    fn perform_boss_attack(boss: &mut Boss) -> i64 {
        let boss_damage = Self::get_random_value(10_000, 20_000);
        boss.stats.total_damage_dealt += boss_damage;

        boss_damage 
    }

    fn update_damage_taken(player: &mut Player, boss_damage: i64, total_taken_damage: i64, ) {
        player.stats.damage_taken += boss_damage;
        player.stats.damage_taken_percentage = player.stats.damage_taken as f32 / total_taken_damage as f32;
    }

    fn seconds_to_mm_ss(seconds: i64) -> String {
        let minutes = seconds / 60;

        if minutes > 60 {
            return "∞".into();
        }

        let remaining_seconds = seconds % 60;
        format!("{:02}:{:02}", minutes, remaining_seconds)
    }

    fn recalculate_all_stats(&mut self, updated_on: DateTime<Utc>, duration_seconds: i64) {
        for player in self.players.iter_mut() {
            let death_log = &mut player.entity.death_log;
            
            if death_log.is_dead {
                let duration = death_log.recorded_on.map(|recorded_on| updated_on - recorded_on);
                death_log.duration = duration;
            }
            
            let stats = &mut player.entity.stats;
            stats.dps = Dps::new(stats.total_damage, duration_seconds);
        }
    }

    fn update_player(
        player: &mut Player,
        attack_result: &AttackResult,
        total_damage: i64,
        updated_on: DateTime<Utc>) {
        let damage = attack_result.damage;
        let stats = &mut player.stats;

        stats.top_damage = max(attack_result.damage, stats.top_damage);
        stats.total_damage += attack_result.damage;
        
        stats.skills.hit_count += 1;
        stats.crit_rate = stats.skills.crit_count as f32 / stats.skills.hit_count as f32;
        let skill_id = attack_result.skill.id;
        let skill_stat = stats.skills.skill.entry(skill_id.to_string()).or_insert_with(|| {
            let mut skill_stat = PlayerSkillStats::default();

            skill_stat.id = skill_id;
            skill_stat.name = attack_result.skill.name.clone();
            skill_stat.icon = attack_result.skill.icon.clone();

            skill_stat
        });

        if attack_result.hit_flag == HitFlag::Critical {
            stats.crit_damage += damage;
            stats.skills.crit_count += 1;
            
            skill_stat.crit_count += 1;
            skill_stat.crit_damage += damage;
        }

        skill_stat.total_damage += damage;
        skill_stat.hit_count += 1;

        match attack_result.hit_option {
            HitOption::None => stats.non_positional_attacks_total_damage += damage,
            HitOption::BackAttack => stats.back_attacks_total_damage += damage,
            HitOption::FrontalAttack => stats.front_attacks_total_damage += damage,
            HitOption::FlankAttack => stats.non_positional_attacks_total_damage += damage,
            HitOption::Max => stats.non_positional_attacks_total_damage += damage,
        }

        stats.damage_percentage = stats.total_damage as f32 / total_damage as f32;
        stats.front_attacks_damage_percentage = stats.front_attacks_total_damage as f32 / stats.total_damage as f32;
        stats.back_attacks_damage_percentage = stats.back_attacks_total_damage as f32 / stats.total_damage as f32;
        stats.non_positional_attacks_damage_percentage = stats.non_positional_attacks_total_damage as f32 / stats.total_damage as f32;
        stats.updated_on = updated_on;
        // info!("{:?}", stats);
    }

    fn perform_attack<'b>(current_boss_hp: i64, template: &'b PlayerTemplate) -> AttackResult<'b> {
        let mut has_fight_ended = false;

        let mut damage = if current_boss_hp < template.max_dmg {
            has_fight_ended = true;
            current_boss_hp
        }
        else
        {
            Self::get_random_value(template.min_dmg, template.max_dmg)
        };

        let skill = Self::get_random_item(&template.skills);

        let mut hit_flag = HitFlag::Normal;
        let hit_option = random_hit_option();

        if Self::get_random_f32_value(0.0, 1.0) <= template.crit_rate {
            damage *= 2;
            hit_flag = HitFlag::Critical;
        }

        if current_boss_hp < damage {
            has_fight_ended = true;
            damage = current_boss_hp;
        };

        let result = AttackResult {
            skill,
            has_fight_ended,
            hit_flag,
            hit_option,
            damage,
        };

        result
    }

    pub fn create_boss(&mut self, npc_id: NpcId, name: &str, max_hp: i64, hp_bars: i64) {
        let now = Utc::now();

        let boss = Boss {
            id: self.get_random_unique_entity_id(),
            npc_id,
            name: name.into(),
            stats: BossStats {
                max_hp: max_hp,
                hp: max_hp,
                max_hp_bars: hp_bars,
                hp_bars: hp_bars,
                hp_percentage: 100.0,
                hp_per_bar: ((max_hp as f64) / (hp_bars as f64)).floor() as f32,
                updated_on: now,
                ..Default::default()
            },
            created_on: now
        };

        self.boss = Some(boss);
    }
    
    fn get_random_skills_for_class(class_skills: &mut Vec<&RawSkill>) -> Vec<Skill> {
        let mut rng = rand::thread_rng();
        class_skills.shuffle(&mut rng);

        let random_8: Vec<_> = class_skills.iter().take(8).cloned().collect();

        let mut skills = vec![];

        for raw_skill in random_8 {
            let skill = Skill {
                id: raw_skill.id,
                icon: raw_skill.icon.unwrap().to_string(),
                name: raw_skill.name.unwrap().to_string()
            };

            skills.push(skill);
        }

        skills
    }

    fn update_boss(stats: &mut BossStats, attack_result: &AttackResult, updated_on: DateTime<Utc>) {

        stats.hp -= attack_result.damage;
        stats.hp_percentage = (stats.hp as f32) / (stats.max_hp as f32) * 100.0;
        stats.hp_bars = (stats.hp as f32 / stats.hp_per_bar).floor() as i64;
        stats.damage_taken = attack_result.damage;
        stats.updated_on = updated_on;
    }

    pub fn to_fight_update_event(&self) -> impl AppEvent {
        let mut players: Vec<Player> = self.players.iter().map(|player| player.entity.clone()).collect();
        players.sort_unstable_by(|a, b| a.stats.total_damage.cmp(&b.stats.total_damage));

        let esthers = self.esthers.iter().map(|esther| esther.entity.clone()).collect();

        let app_event = FightUpdate {
            esthers,
            stats: self.stats.clone(),
            boss: self.boss.clone().unwrap(),
            players
        };

        app_event
    }

    pub fn generate_random_range(size: u32) -> (u32, u32) {
        let mut rng = rand::thread_rng();
        let start = 2;
        let end = rng.gen_range(start..=size);

        (start, end)
    }

    pub fn get_random_alive_player(players: &mut Vec<PlayerWithTemplate>) -> &mut Player {
        let players_count = players.len();
        let mut player_index = Self::get_random_index(players_count);
        let player = players.get_mut(player_index).unwrap();

        while player.entity.death_log.is_dead {
            player_index = Self::get_random_index(players_count);
        }

        let player = players.get_mut(player_index).unwrap();

        &mut player.entity
    }
    

    fn get_random_esther(esthers: &mut Vec<EstherWithTemplate>) -> &mut EstherWithTemplate {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..esthers.len());
    
        &mut esthers[random_index]
    }

    fn get_random_item<T>(items: &[T]) -> &T {
        debug!("random_item");
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..items.len());
    
        &items[random_index]
    }

    fn get_random_index(size: usize) -> usize {
        debug!("random_index");
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..size);
        index
    }

    fn get_random_value(min: i64, max: i64) -> i64 {
        debug!("random_value");
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }

    fn get_random_f32_value(min: f32, max: f32) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }

    fn get_random_nickname(&self, length: usize) -> String {
        let mut rng = rand::thread_rng();
        let mut result = String::new();
    
        if length > 0 {
            let first_char = rng.gen_range(b'A'..=b'Z') as char;
            result.push(first_char);
    
            for _ in 1..length {
                let next_char = rng.gen_range(b'a'..=b'z') as char;
                result.push(next_char);
            }
        }
    
        result
    }

    fn get_random_unique_entity_id(&mut self) -> EntityId {
        let min = 10000;
        let max = 100000;
        let mut rng = rand::thread_rng();
        let mut value;

        loop {
            value = rng.gen_range(min..=max);

            if !self.registered_ids.contains(&value) {
                self.registered_ids.insert(value);
                break;
            }
        }

        value
    }

    fn get_random_dps_class(dps_classes: &Vec<&'a Class>, registered_dps_class_ids: &mut HashSet<ClassId>) -> &'a Class<'a> {
        let mut rng = rand::thread_rng();
        let mut random_index = rng.gen_range(0..dps_classes.len());
        let mut dps_class;

        loop {
            dps_class = dps_classes[random_index];
            let class_id = dps_class.id;

            if !registered_dps_class_ids.contains(&class_id) {
                registered_dps_class_ids.insert(class_id);
                break;
            }

            random_index = rng.gen_range(0..dps_classes.len());
        }

        dps_class
    }

    fn get_random_sup_class(&self) -> &Class {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.sup_classes.len());
    
        self.sup_classes[random_index]
    }
}