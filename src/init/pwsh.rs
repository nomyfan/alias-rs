use crate::model::{Alias, AliasConfig};

const PWSH_SCRIPT: &str = include_str!("./pwsh.ps1");

pub fn init(config: AliasConfig) {
    println!("{PWSH_SCRIPT}");

    for (fn_name, value) in config.aliases.iter() {
        match value {
            Alias::Inline(inline) => {
                println!("function {fn_name} {{ {inline} }}");
            }
            Alias::Multi(multi) => {
                let fn_body = multi.join(" ");
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
