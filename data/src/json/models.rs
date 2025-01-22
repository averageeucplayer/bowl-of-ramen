use std::collections::HashMap;

use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use crate::deserializer::*;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Class<'a> {
    pub id: u32,
    pub name: &'a str,
    pub is_generic: bool,
    pub is_support: bool
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawSkill<'a> {
    pub id: u32,
    pub name: Option<&'a str>,
    pub desc: Option<String>,
    #[serde(deserialize_with = "u32_zero_as_none")]
    pub class_id: Option<u32>,
    pub icon: Option<&'a str>,
    pub grade: SkillGrade,
}

#[derive(Debug, Default, Deserialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SkillGrade {
    #[default]
    Unknown,
    Normal,
    Super,
    Awakening,
}