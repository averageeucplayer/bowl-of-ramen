pub mod models;

use std::collections::HashMap;
use models::Class;
use once_cell::sync::Lazy;

pub static CLASS_MAP: Lazy<HashMap<u32, Class>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./Class.json");
    serde_json::from_slice(json_bytes).unwrap()
});