use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Class<'a> {
    pub id: u32,
    pub name: &'a str,
    pub is_support: bool
}