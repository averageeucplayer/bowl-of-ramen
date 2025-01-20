use app_core::models::*;
use std::{cmp::max, collections::HashSet};

use chrono::{DateTime, TimeDelta, Utc};
use data::json::{models::Class, CLASS_MAP};
use log::{debug, info};
use rand::Rng;
use serde::Serialize;

const HIT_OPTIONS: [HitOption; 3] = [HitOption::BackAttack, HitOption::FlankAttack, HitOption::FrontalAttack];

pub fn random_hit_option() -> HitOption {
    let random_index = rand::thread_rng().gen_range(0..HIT_OPTIONS.len());
    HIT_OPTIONS[random_index]
}

#[derive(Debug)]
pub struct AttackResult {
    pub has_fight_ended: bool,
    pub hit_flag: HitFlag,
    pub hit_option: HitOption,
    pub damage: i64
}

#[derive(Debug)]
pub struct PlayerWithTemplate {
    pub player: Player,
    pub template: PlayerTemplate
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerTemplate {
    crit_rate: f32,
    min_dmg: i64,
    max_dmg: i64
}

#[derive(Debug, Default)]
pub struct FightSimulator<'a> {
    registered_ids: HashSet<EntityId>,
    dps_classes: Vec<&'a Class<'a>>,
    sup_classes: Vec<&'a Class<'a>>,
    registered_dps_class_ids: HashSet<ClassId>,
    players: Vec<PlayerWithTemplate>,
    boss: Option<Boss>,
    has_fight_ended: bool,
    fight_started_on: Option<DateTime<Utc>>,
    duration: TimeDelta,
}

impl<'a> FightSimulator<'a> {
    pub fn new() -> Self {
        let dps_classes = CLASS_MAP.values()
            .filter(|class| !class.is_support)
            .collect();

        let sup_classes = CLASS_MAP.values()
            .filter(|class| class.is_support)
            .collect();

        Self {
            dps_classes,
            sup_classes,
            ..Default::default()
        }
    }

    pub fn create_8_players(&mut self, min_dmg: i64, max_dmg: i64) {
        let now = Utc::now();
        let mut players =  vec![];

        for index in 1..9 {

            let id = self.random_unique_entity_id();
            let name = self.random_nickname(10);

            let class = if index % 4 == 0 {
                self.random_sup_class()
            }
            else {
                self.random_dps_class()
            };

            let template = if class.is_support {
                PlayerTemplate {
                    crit_rate: 0.1,
                    min_dmg: 10_000,
                    max_dmg: 100_000,
                }
            }
            else {
                PlayerTemplate {
                    crit_rate: Self::random_f32_value(0.5, 1.0),
                    min_dmg,
                    max_dmg,
                }
            };

            let player = Player {
                id,
                name,
                class_name: class.name.to_string(),
                class_id: class.id,
                stats: PlayerStats::default(),
                created_on: now,
            };

            let player_with_template = PlayerWithTemplate {
                player,
                template
            };

            players.push(player_with_template);
        }

        self.players = players;
    }

    pub fn create_player(&mut self) {

    }

    pub fn has_ended(&self) -> bool {
        self.has_fight_ended
    }
    
    pub fn perform_attack_and_update_stats(&mut self) {
        
        let now = Utc::now();
        let fight_started_on = self.fight_started_on.get_or_insert_with(|| now);
        
        self.duration = *fight_started_on - now;
        let duration_seconds = self.duration.num_seconds();

        let boss = self.boss.as_ref().unwrap();
        let player_index = self.random_player();
        let player = self.players.get_mut(player_index).unwrap();
        let result = Self::perform_attack(boss, &player.template);
        
        if result.has_fight_ended {
            self.has_fight_ended = true;
        }

        // debug!("{:?}", result);
    
        Self::update_player(&mut player.player, &result, duration_seconds, now);
        self.update_boss(&result, now);
        self.recalculate_dps(duration_seconds);
    }

    fn recalculate_dps(&mut self, duration_seconds: i64) {
        for player in self.players.iter_mut() {
            let stats = &mut player.player.stats;
            stats.dps = Dps::new(stats.total_damage / duration_seconds);
        }
    }

    fn update_player(
        player: &mut Player,
        attack_result: &AttackResult,
        duration_seconds: i64,
        updated_on: DateTime<Utc>) {
        let damage = attack_result.damage;
        let stats = &mut player.stats;

        stats.top_damage = max(attack_result.damage, stats.top_damage);
        stats.total_damage += attack_result.damage;
        
        if attack_result.hit_flag == HitFlag::Critical {
            stats.crit_damage += damage;
            stats.crit_count += 1;
        }

        stats.hit_count += 1;

        stats.crit_rate = stats.crit_count as f32 / stats.hit_count as f32;

        match attack_result.hit_option {
            HitOption::None => stats.non_positional_attacks_total_damage += damage,
            HitOption::BackAttack => stats.back_attacks_total_damage += damage,
            HitOption::FrontalAttack => stats.front_attacks_total_damage += damage,
            HitOption::FlankAttack => stats.non_positional_attacks_total_damage += damage,
            HitOption::Max => stats.non_positional_attacks_total_damage += damage,
        }

        stats.dps = if duration_seconds > 0 {
            Dps::new(stats.total_damage / duration_seconds)
        }
        else {
            Dps::default()
        };

        stats.front_attacks_damage_percentage = stats.front_attacks_total_damage as f32 / stats.total_damage as f32;
        stats.back_attacks_damage_percentage = stats.back_attacks_total_damage as f32 / stats.total_damage as f32;
        stats.non_positional_attacks_damage_percentage = stats.non_positional_attacks_total_damage as f32 / stats.total_damage as f32;
        stats.updated_on = updated_on;
        // info!("{:?}", stats);
    }

    fn perform_attack(boss: &Boss, template: &PlayerTemplate) -> AttackResult {
        let mut has_fight_ended = false;

        let mut damage = if boss.stats.hp < template.max_dmg {
            has_fight_ended = true;
            boss.stats.hp
        }
        else
        {
            Self::random_value(template.min_dmg, template.max_dmg)
        };

        let mut hit_flag = HitFlag::Normal;
        let hit_option = random_hit_option();

        if Self::random_f32_value(0.0, 1.0) <= template.crit_rate {
            damage *= 2;
            hit_flag = HitFlag::Critical;
        }

        if boss.stats.hp < damage {
            has_fight_ended = true;
            damage = boss.stats.hp;
        };

        let result = AttackResult {
            has_fight_ended,
            hit_flag,
            hit_option,
            damage,
        };

        result
    }

    pub fn create_boss(&mut self, name: &str, max_hp: i64, hp_bars: i64) {
        let now = Utc::now();

        let boss = Boss {
            id: self.random_unique_entity_id(),
            name: name.into(),
            stats: BossStats {
                max_hp: max_hp,
                hp: max_hp,
                max_hp_bars: hp_bars,
                hp_bars: hp_bars,
                hp_percentage: 100.0,
                hp_per_bar: ((max_hp as f64) / (hp_bars as f64)).floor() as f32,
                damage_taken: 0,
                updated_on: Utc::now()
            },
            created_on: now
        };

        self.boss = Some(boss);
    }
    
    fn update_boss(&mut self, attack_result: &AttackResult, updated_on: DateTime<Utc>) {
        let boss = self.boss.as_mut().unwrap();
        let stats = &mut boss.stats;

        stats.hp -= attack_result.damage;
        stats.hp_percentage = (stats.hp as f32) / (stats.max_hp as f32) * 100.0;
        stats.hp_bars = (stats.hp as f32 / stats.hp_per_bar).floor() as i64;
        stats.damage_taken = attack_result.damage;
        stats.updated_on = updated_on;
    }

    pub fn to_fight_update_event(&self) -> impl AppEvent {
        let players = self.players.iter().map(|player| player.player.clone()).collect();

        let app_event = FightUpdate {
            boss: self.boss.clone().unwrap(),
            players: players
        };

        app_event
    }

    fn random_player(&self) -> usize {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.players.len());
        random_index
    }

    fn random_value(min: i64, max: i64) -> i64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }

    fn random_f32_value(min: f32, max: f32) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }

    fn random_nickname(&self, length: usize) -> String {
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

    fn random_unique_entity_id(&mut self) -> EntityId {
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

    fn random_dps_class(&mut self) -> &Class {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.dps_classes.len());
        let mut dps_class;

        loop {
            dps_class = self.dps_classes[random_index];
            let class_id = dps_class.id;

            if !self.registered_dps_class_ids.contains(&class_id) {
                self.registered_dps_class_ids.insert(class_id);
                break;
            }
        }

        dps_class
    }

    fn random_sup_class(&self) -> &Class {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.sup_classes.len());
    
        self.sup_classes[random_index]
    }
}