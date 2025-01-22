use std::{collections::HashMap, fmt::format};

use app_macros::AppEvent;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::abbreviate_number;

pub type EntityId = u64;
pub type ClassId = u32;
pub type SkillId = u32;
pub type NpcId = u32;

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
    pub npc_id: NpcId,
    pub name: String,
    pub stats: BossStats,
    pub created_on: DateTime<Utc>
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BossStats {
    pub max_hp: i64,
    pub hp: i64,
    pub shield_hp: i64,
    pub max_hp_bars: i64,
    pub hp_bars: i64,
    pub hp_per_bar: f32,
    pub hp_percentage: f32,
    pub damage_taken: i64,
    pub total_damage_dealt: i64,
    pub updated_on: DateTime<Utc>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub id: EntityId,
    pub name: String,
    pub class_color: String,
    pub class_name: String,
    pub class_id: ClassId,
    pub created_on: DateTime<Utc>,
    pub stats: PlayerStats,
    pub death_log: DeathLog,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeathLog {
    pub is_dead: bool,
    pub death_count: u32,
    pub recorded_on: Option<DateTime<Utc>>,
    pub duration: Option<Duration>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Esther {
    pub id: EntityId,
    pub name: String,
    pub icon: String,
    pub stats: EstherStats
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EstherStats {
    pub total_damage: i64,
    pub damage_percentage: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Skill {
    pub id: SkillId,
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncounterStats {
    pub duration_mmss: String,
    pub time_to_kill_mmss: String,
    pub party_dps: Dps,
    pub top_damage: i64,
    pub total_player_damage: i64,
    pub total_esther_damage: i64,
    pub total_taken_damage: i64,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Deserialize)]
pub struct PlayerStats {
    pub crit_rate: f32,
    pub crit_damage: i64,
    pub top_damage: i64,
    pub total_damage: i64,
    pub damage_percentage: f32,
    pub dps: Dps,
    pub back_attacks_total_damage: i64,
    pub front_attacks_total_damage: i64,
    pub non_positional_attacks_total_damage: i64,
    pub back_attacks_damage_percentage: f32,
    pub front_attacks_damage_percentage: f32,
    pub non_positional_attacks_damage_percentage: f32,
    pub hyper_awakening_damage: i64,
    pub updated_on: DateTime<Utc>,
    pub damage_taken: i64,
    pub damage_taken_percentage: f32,
    pub skills: PlayerSkillsStats
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Deserialize)]
pub struct PlayerSkillsStats {
    pub counter_count: i64,
    pub hit_count: u32,
    pub crit_count: u32,
    pub skill: HashMap<String, PlayerSkillStats>
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Deserialize)]
pub struct PlayerSkillStats {
    pub id: SkillId,
    pub name: String,
    pub icon: String,
    pub hit_count: u32,
    pub crit_count: u32,
    pub crit_damage: i64,
    pub total_damage: i64,
    // pub cast_log: Vec<SkillCastLog>
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Deserialize)]
pub struct SkillCastLog {
    pub first_recorded_on: i64,
    pub last_recorded_on: i64,
    pub hits: Vec<SkillHit>
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Deserialize)]
pub struct SkillHit {
    pub recorded_on: i64,
    pub damage: i64,
    pub is_critical: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct Dps {
    pub raw: i64,
    pub abbreviated: String,
    pub value: f64,
    pub unit: char
}

impl Dps {
    pub fn new(total_damage: i64, duration_seconds: i64) -> Self {
        if duration_seconds == 0 {
            return Self::default()
        }

        let raw = total_damage / duration_seconds;
        let (value, unit) = abbreviate_number(raw);

        Self {
            raw,
            abbreviated: format!("{:.1}{}", value, unit),
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
    pub stats: EncounterStats,
    pub esthers: Vec<Esther>,
    pub players: Vec<Player>,
    pub boss: Boss
}