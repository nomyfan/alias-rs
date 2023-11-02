use crate::model::{Alias, AliasConfig};

const PWSH_SCRIPT: &str = include_str!("./pwsh.ps1");
const ZSH_SCRIPT: &str = include_str!("./zsh.zsh");

pub(crate) fn init(config: AliasConfig, args: Vec<String>) {
    let shell = args.get(0).unwrap().as_ref();
    match shell {
        "powershell" => {
            for (fn_name, value) in config.aliases.iter() {
                if let Alias::Object(v) = value {
                    if let Some(fn_body) = v.get("powershell") {
                        println!("function {fn_name} {{ {fn_body} }}");
                    }
                }
            }
            print!("{PWSH_SCRIPT}")
        }
        "zsh" => {
            // TODO: init alias as function
            print!("{ZSH_SCRIPT}")
        }
        _ => unreachable!("Unsupported shell"),
    }
}
