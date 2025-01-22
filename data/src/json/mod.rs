pub mod models;

use app_core::settings::Settings;
use models::*;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

pub static CLASS_MAP: Lazy<FxHashMap<u32, Class>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./Class.json");
    serde_json::from_slice(json_bytes).unwrap()
});

pub static SKILL_MAP: Lazy<FxHashMap<u32, RawSkill>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./Skill.json");
    serde_json::from_slice(json_bytes).unwrap()
});

pub static DEFAULT_SETTINGS: Lazy<Settings> = Lazy::new(|| {
    let json_bytes = include_bytes!("./DefaultSettings.json");
    serde_json::from_slice(json_bytes).unwrap()
});