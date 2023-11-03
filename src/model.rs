use std::collections::HashMap;
use toml::{Table, Value};

pub type Aliases = HashMap<String, AliasValue>;

#[derive(Debug, Default)]
pub struct AliasConfig {
    pub aliases: Aliases,
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
            aliases.insert(key.clone(), AliasValue::try_from(value)?);
        }

        Ok(Self { aliases })
    }

    pub fn visit_aliases<V>(&self, shell_name: &str, mut visitor: V)
    where
        V: AliasVisitor,
    {
        for (alias_name, alias_value) in self.aliases.iter() {
            match alias_value {
                AliasValue::Inline(value) => {
                    visitor.visit((alias_name, VisitorAliasValue::Inline(value)));
                }
                AliasValue::Multi(value) => {
                    visitor.visit((alias_name, VisitorAliasValue::Multi(value)));
                }
                AliasValue::Object(object) => {
                    if let Some(value) = object.get(shell_name) {
                        match value {
                            AliasObjectValue::Inline(value) => {
                                visitor.visit((alias_name, VisitorAliasValue::Inline(value)));
                            }
                            AliasObjectValue::Multi(value) => {
                                visitor.visit((alias_name, VisitorAliasValue::Multi(value)));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum AliasValue {
    Inline(String),
    Multi(Vec<String>),
    Object(HashMap<String, AliasObjectValue>),
}

impl TryFrom<&Value> for AliasValue {
    type Error = anyhow::Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(value) => Ok(AliasValue::Inline(value.to_string())),
            Value::Array(value) => Ok(AliasValue::Multi(
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

                Ok(AliasValue::Object(platform_aliases))
            }
            _ => anyhow::bail!("Unsupported value type"),
        }
    }
}

#[derive(Debug)]
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

pub enum VisitorAliasValue<'a> {
    Inline(&'a str),
    Multi(&'a Vec<String>),
}

impl std::fmt::Display for VisitorAliasValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisitorAliasValue::Inline(value) => write!(f, "{}", value),
            VisitorAliasValue::Multi(value) => write!(f, "{}", value.join(" ")),
        }
    }
}

pub trait AliasVisitor {
    fn visit<'a>(&mut self, alias: (&'a str, VisitorAliasValue<'a>));
}
