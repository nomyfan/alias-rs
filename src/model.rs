use serde::Deserialize;
use std::collections::HashMap;
use toml::{Table, Value};

#[derive(Deserialize, Debug, Default)]
pub struct AliasConfig {
    pub aliases: HashMap<String, Alias>,
}

impl AliasConfig {
    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        let table: Table = toml::from_str(s)?;
        let mut aliases = HashMap::new();

        let aliases_table = table
            .get("aliases")
            .and_then(|aliases| aliases.as_table())
            .ok_or_else(|| anyhow::anyhow!("No valid aliases found"))?;
        for (key, value) in aliases_table {
            aliases.insert(key.clone(), Alias::try_from(value)?);
        }

        Ok(Self { aliases })
    }
}

#[derive(Deserialize, Debug)]
pub enum Alias {
    Inline(String),
    Multi(Vec<String>),
    Object(HashMap<String, AliasObjectValue>),
}

impl TryFrom<&Value> for Alias {
    type Error = anyhow::Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(value) => Ok(Alias::Inline(value.to_string())),
            Value::Array(value) => Ok(Alias::Multi(
                value
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect::<Vec<_>>(),
            )),
            Value::Table(value) => {
                let mut platform_aliases = HashMap::new();
                for (key, value) in value {
                    match value {
                        Value::String(value) => {
                            platform_aliases
                                .insert(key.into(), AliasObjectValue::Inline(value.to_string()));
                        }
                        Value::Array(value) => {
                            platform_aliases.insert(
                                key.into(),
                                AliasObjectValue::Multi(
                                    value
                                        .iter()
                                        .map(|x| x.as_str().unwrap().to_string())
                                        .collect::<Vec<_>>(),
                                ),
                            );
                        }
                        _ => anyhow::bail!("Unsupported value type"),
                    }
                }

                Ok(Alias::Object(platform_aliases))
            }
            _ => anyhow::bail!("Unsupported value type"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum AliasObjectValue {
    Inline(String),
    Multi(Vec<String>),
}

impl std::fmt::Display for AliasObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AliasObjectValue::Inline(value) => write!(f, "{}", value),
            AliasObjectValue::Multi(value) => write!(f, "{}", value.join(" ")),
        }
    }
}
