use super::bash::bash_like;
use crate::model::AliasConfig;

const ZSH_SCRIPT: &str = include_str!("./zsh.zsh");

pub fn init(config: AliasConfig) {
    println!("{ZSH_SCRIPT}");

    bash_like("zsh", &config.aliases).iter().for_each(|x| {
        println!("{}", x);
    });
}
