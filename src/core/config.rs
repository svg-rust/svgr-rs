use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Icon {
    Bool(bool),
    Str(String),
    Num(f64),
}

impl Default for Icon {
    fn default() -> Self {
        Icon::Bool(false)
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub ref_: Option<bool>,

    #[serde(default)]
    pub icon: Option<Icon>
}
