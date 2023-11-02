mod pwsh;

use crate::model::AliasConfig;

const ZSH_SCRIPT: &str = include_str!("./zsh.zsh");

pub(crate) fn init(config: AliasConfig, args: Vec<String>) {
    let shell = args.get(0).unwrap().as_ref();
    match shell {
        "powershell" => {
            pwsh::init(config);
        }
        "zsh" => {
            // TODO: init alias as function
            print!("{ZSH_SCRIPT}")
        }
        _ => unreachable!("Unsupported shell"),
    }
}
