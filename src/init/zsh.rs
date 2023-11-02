use crate::model::{Alias, AliasConfig};

const ZSH_SCRIPT: &str = include_str!("./zsh.zsh");

pub fn init(config: AliasConfig) {
    println!("{ZSH_SCRIPT}");

    for (alias_name, alias_value) in config.aliases.iter() {
        match alias_value {
            Alias::Inline(value) => {
                println!("alias {alias_name}=\"{value}\"");
            }
            Alias::Multi(value) => {
                let value = value.join(" ");
                println!("alias {alias_name}=\"{value}\"");
            }
            Alias::Object(object) => {
                if let Some(value) = object.get("zsh") {
                    println!("alias {alias_name}=\"{value}\"");
                }
            }
        }
    }
}
