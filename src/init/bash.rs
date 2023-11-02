use crate::model::{Alias, AliasConfig, Aliases};

const BASH_SCRIPT: &str = include_str!("./bash.bash");

#[inline]
fn alias<V: std::fmt::Display>(name: &str, value: V) -> String {
    format!("alias {}=\"{}\"", name, value)
}

pub fn bash_like(shell_name: &str, aliases: &Aliases) -> Vec<String> {
    let mut output = Vec::new();
    for (alias_name, alias_value) in aliases.iter() {
        match alias_value {
            Alias::Inline(value) => {
                output.push(alias(alias_name, value));
            }
            Alias::Multi(value) => {
                output.push(alias(alias_name, value.join(" ")));
            }
            Alias::Object(object) => {
                if let Some(value) = object.get(shell_name) {
                    output.push(alias(alias_name, value));
                }
            }
        }
    }

    output
}

pub fn init(config: AliasConfig) {
    println!("{BASH_SCRIPT}");

    bash_like("bash", &config.aliases).iter().for_each(|x| {
        println!("{}", x);
    });
}
