mod bash;
mod pwsh;
mod zsh;

use crate::model::AliasConfig;

pub(crate) fn init(config: AliasConfig, args: Vec<String>) {
    let shell = args.get(0).unwrap().as_ref();
    match shell {
        "powershell" => {
            pwsh::init(config);
        }
        "zsh" => {
            zsh::init(config);
        }
        "bash" => {
            bash::init(config);
        }
        _ => unreachable!("Unsupported shell"),
    }
}
