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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExpandProps {
    Bool(bool),
    Start,
    End
}

impl Default for ExpandProps {
    fn default() -> Self {
        ExpandProps::End
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JSXRuntime {
    Classic,
    ClassicPreact,
    Automatic,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JSXRuntimeImport {
    pub source: String,
    pub namespace: Option<String>,
    pub default_specifier: Option<String>,
    pub specifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    #[serde(rename(serialize = "ref"))]
    pub _ref: Option<bool>,

    #[serde(default)]
    pub title_prop: Option<bool>,

    #[serde(default)]
    pub desc_prop: Option<bool>,

    #[serde(default)]
    pub expand_props: Option<ExpandProps>,

    #[serde(default)]
    pub dimensions: Option<bool>,

    #[serde(default)]
    pub icon: Option<Icon>,

    #[serde(default)]
    pub native: Option<bool>,

    #[serde(default)]
    pub typescript: Option<bool>,

    #[serde(default)]
    pub memo: Option<bool>,

    #[serde(default)]
    pub jsx_runtime: Option<JSXRuntime>,

    #[serde(default)]
    pub jsx_runtime_import: Option<JSXRuntimeImport>,

    #[serde(default)]
    pub named_export: Option<String>,
}
