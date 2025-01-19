use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Deserialize)]
pub struct FightUpdate {
    pub players: Vec<Player>
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Player {
    pub id: EntityId,
    pub name: String,
    pub class_id: ClassId,
    pub stats: PlayerStats,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Deserialize)]
pub struct PlayerStats {
    pub crit_rate: f32,
    pub crit_damage: i64,
    pub top_damage: i64,
    pub total_damage: i64,
    pub back_attacks: i64,
    pub front_attacks: i64,
    pub non_positional_attacks: i64,
    pub updated_on: DateTime<Utc>
}

