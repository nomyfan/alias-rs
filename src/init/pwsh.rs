use crate::model::{Alias, AliasConfig};

const PWSH_SCRIPT: &str = include_str!("./pwsh.ps1");

pub fn init(config: AliasConfig) {
    println!("{PWSH_SCRIPT}");

    for (fn_name, alias_value) in config.aliases.iter() {
        match alias_value {
            Alias::Inline(value) => {
                println!("function {fn_name} {{ {value} }}");
            }
            Alias::Multi(value) => {
                let fn_body = value.join(" ");
                println!("function {fn_name} {{ {fn_body} }}");
            }
            Alias::Object(object) => {
                if let Some(fn_body) = object.get("powershell") {
                    println!("function {fn_name} {{ {fn_body} }}");
                }
            }
        }
    }
}
