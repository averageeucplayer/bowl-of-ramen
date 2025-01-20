use std::fmt::format;

use app_macros::AppEvent;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::abbreviate_number;

pub type EntityId = u64;
pub type ClassId = u32;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum HitFlag {
    Normal = 0,
    Critical = 1,
    Miss = 2,
    Invincible = 3,
    DamageOverTime = 4,
    Immune = 5,
    ImmuneSilenced = 6,
    FontSilenced = 7,
    DamageOverTimeCritical = 8,
    Dodge = 9,
    Reflect = 10,
    DamageShare = 11,
    DodgeHit = 12,
    Max = 13,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum HitOption {
    None,
    BackAttack,
    FrontalAttack,
    FlankAttack,
    Max,
}

pub trait AppEvent: Serialize + Clone {
    fn event_name(&self) -> &'static str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Boss {
    pub id: EntityId,
    pub name: String,
    pub stats: BossStats,
    pub created_on: DateTime<Utc>
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BossStats {
    pub max_hp: i64,
    pub hp: i64,
    pub max_hp_bars: i64,
    pub hp_bars: i64,
    pub hp_per_bar: f32,
    pub hp_percentage: f32,
    pub damage_taken: i64,
    pub updated_on: DateTime<Utc>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub id: EntityId,
    pub name: String,
    pub class_name: String,
    pub class_id: ClassId,
    pub created_on: DateTime<Utc>,
    pub stats: PlayerStats,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Deserialize)]
pub struct PlayerStats {
    pub crit_rate: f32,
    pub crit_damage: i64,
    pub top_damage: i64,
    pub total_damage: i64,
    pub dps: Dps,
    pub back_attacks_total_damage: i64,
    pub front_attacks_total_damage: i64,
    pub non_positional_attacks_total_damage: i64,
    pub back_attacks_damage_percentage: f32,
    pub front_attacks_damage_percentage: f32,
    pub non_positional_attacks_damage_percentage: f32,
    pub updated_on: DateTime<Utc>,
    pub hit_count: u32,
    pub crit_count: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct Dps {
    pub raw: i64,
    pub abbreviated: String,
    pub value: f64,
    pub unit: char
}

impl Dps {
    pub fn new(raw: i64) -> Self {
        let (value, unit) = abbreviate_number(raw);

        Self {
            raw,
            abbreviated: format!("{}{}", value, unit),
            value,
            unit
        }
    }
}

impl Default for Dps {
    fn default() -> Self {
        Dps {
            raw: 0,
            abbreviated: String::from("0.0"),
            value: 0.0,
            unit: ' ',
        }
    }
}

#[derive(Debug, AppEvent, Clone, Serialize, Deserialize)]
pub struct FightUpdate {
    pub players: Vec<Player>,
    pub boss: Boss
}