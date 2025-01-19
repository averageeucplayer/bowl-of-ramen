use app_core::models::{ClassId, EntityId, HitFlag, HitOption, PlayerStats};
use std::{cmp::max, collections::HashSet};

use chrono::{DateTime, Utc};
use data::json::CLASS_MAP;
use log::{debug, info};
use rand::Rng;
use serde::Serialize;

use crate::misc::AppEvent;

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

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    id: EntityId,
    name: String,
    class_id: ClassId,
    stats: PlayerStats,
    template: PlayerTemplate
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerTemplate {
    crit_rate: f32,
    min_dmg: i64,
    max_dmg: i64
}


#[derive(Debug, Clone, Serialize)]
pub struct Boss<'a> {
    id: EntityId,
    name: &'a str,
    stats: BossStats,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BossStats {
    max_hp: i64,
    hp: i64,
    max_hp_bars: i64,
    hp_bars: i64,
    hp_per_bar: f32,
    hp_percentage: f32,
    damage_taken: i64,
    updated_on: DateTime<Utc>
}

#[derive(Debug, Default)]
pub struct FightSimulator<'a> {
    registered_ids: HashSet<EntityId>,
    dps_class_ids: Vec<ClassId>,
    registered_dps_class_ids: HashSet<ClassId>,
    sup_class_ids: Vec<ClassId>,
    players: Vec<Player>,
    boss: Option<Boss<'a>>,
    has_fight_ended: bool,
}

impl<'a> FightSimulator<'a> {
    pub fn new() -> Self {
        let dps_class_ids = CLASS_MAP.values()
            .filter(|class| !class.is_support)
            .map(|class| class.id)
            .collect();

        let sup_class_ids = CLASS_MAP.values()
            .filter(|class| class.is_support)
            .map(|class| class.id)
            .collect();

        Self {
            dps_class_ids,
            sup_class_ids,
            ..Default::default()
        }
    }

    pub fn create_8_players(&mut self, min_dmg: i64, max_dmg: i64) {

        let mut players =  vec![];

        for index in 1..9 {
            let mut is_support = false;

            let class_id = if index % 4 == 0 {
                is_support = true;
                self.random_sup_class_id()
            }
            else {
                self.random_dps_class_id()
            };

            let template = if is_support {
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
                id: self.random_unique_entity_id(),
                name: self.random_nickname(10),
                class_id,
                stats: PlayerStats::default(),
                template,
            };

            players.push(player);
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

        let boss = self.boss.as_ref().unwrap();
        let player_index = self.random_player();
        let player = self.players.get_mut(player_index).unwrap();
        let result = Self::perform_attack(boss, &player.template);
        
        if result.has_fight_ended {
            self.has_fight_ended = true;
        }

        debug!("{:?}", result);
    
        Self::update_player(player, &result, now);
        self.update_boss(&result, now);
    }

    fn update_player(player: &mut Player, attack_result: &AttackResult, updated_on: DateTime<Utc>) {
        let damage = attack_result.damage;
        let stats = &mut player.stats;

        stats.top_damage = max(attack_result.damage, stats.top_damage);
        stats.total_damage += attack_result.damage;
        
        if attack_result.hit_flag == HitFlag::Critical {
            stats.crit_damage += damage;
        }

        stats.updated_on = updated_on;
        info!("{:?}", stats);
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

    pub fn create_boss(&mut self, name: &'a str, max_hp: i64, hp_bars: i64) {


        let boss = Boss {
            id: self.random_unique_entity_id(),
            name: name,
            stats: BossStats {
                max_hp: max_hp,
                hp: max_hp,
                max_hp_bars: hp_bars,
                hp_bars: hp_bars,
                hp_percentage: 100.0,
                hp_per_bar: ((max_hp as f64) / (hp_bars as f64)).floor() as f32,
                damage_taken: 0,
                updated_on: Utc::now()
            }
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

    pub fn to_fight_update_event(&self) -> AppEvent {
        let app_event = AppEvent::FightUpdate {
            boss: &self.boss.as_ref().unwrap(),
            players: &self.players
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

    fn random_dps_class_id(&mut self) -> ClassId {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.dps_class_ids.len());
        let mut dps_class_id;

        loop {
            dps_class_id = self.dps_class_ids[random_index];

            if !self.registered_dps_class_ids.contains(&dps_class_id) {
                self.registered_dps_class_ids.insert(dps_class_id);
                break;
            }
        }

        dps_class_id
    }

    fn random_sup_class_id(&self) -> ClassId {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.sup_class_ids.len());
    
        self.sup_class_ids[random_index]
    }
}