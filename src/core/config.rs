use serde::{Deserialize, Serialize};
use linked_hash_map::LinkedHashMap;

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

// Untagged enums with empty variants (de)serialize in unintuitive ways
// here: https://github.com/serde-rs/serde/issues/1560
macro_rules! named_unit_variant {
    ($variant:ident) => {
        pub mod $variant {
            pub fn serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let s = stringify!($variant).replace("_", "-");
                serializer.serialize_str(s.as_str())
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V;
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = ();
                    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        let mut s = String::new();
                        s.push_str("\"");
                        s.push_str(stringify!($variant).replace("_", "-").as_str());
                        s.push_str("\"");
                        f.write_str(s.as_str())
                    }
                    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        let s = stringify!($variant).replace("_", "-");
                        if value == s {
                            Ok(())
                        } else {
                            Err(E::invalid_value(serde::de::Unexpected::Str(value), &self))
                        }
                    }
                }
                deserializer.deserialize_str(V)
            }
        }
    };
}

mod strings {
    named_unit_variant!(start);
    named_unit_variant!(end);
    named_unit_variant!(classic);
    named_unit_variant!(classic_preact);
    named_unit_variant!(automatic);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExpandProps {
    Bool(bool),
    #[serde(with = "strings::start")]
    Start,
    #[serde(with = "strings::end")]
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
    #[serde(with = "strings::classic")]
    Classic,
    #[serde(with = "strings::classic_preact")]
    ClassicPreact,
    #[serde(with = "strings::automatic")]
    Automatic,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JSXRuntimeImport {
    pub source: String,
    pub namespace: Option<String>,
    pub default_specifier: Option<String>,
    pub specifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExportType {
    Named,
    Default,
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
    pub expand_props: ExpandProps,

    #[serde(default)]
    pub dimensions: Option<bool>,

    #[serde(default)]
    pub icon: Option<Icon>,

    #[serde(default)]
    pub native: Option<bool>,

    #[serde(default)]
    // Deserialize object/map while maintaining order
    // here: https://github.com/serde-rs/serde/issues/269
    pub svg_props: Option<LinkedHashMap<String, String>>,

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

    #[serde(default)]
    pub export_type: Option<ExportType>,
}
