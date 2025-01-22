use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

use crate::models::ClassId;

#[derive(Debug, Clone, Default, Serialize, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub general: GeneralSettings
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Deserialize)]
pub struct GeneralSettings {
    port: u16,
    always_on_top: bool,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Deserialize)]
pub struct ColorSettings {
    local: String,
    class_colors: FxHashMap<ClassId, String>,
    default_class_colors: FxHashMap<ClassId, String>,
}