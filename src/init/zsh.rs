use super::bash::print_alias;
use crate::model::{AliasConfig, AliasVisitor, VisitorAliasValue};

const ZSH_SCRIPT: &str = include_str!("./zsh.zsh");

struct ZshVisitor {}

impl AliasVisitor for ZshVisitor {
    fn visit<'a>(&mut self, (name, value): (&'a str, VisitorAliasValue<'a>)) {
        print_alias(name, value);
    }
}

pub fn init(config: AliasConfig) {
    println!("{ZSH_SCRIPT}");

    config.visit_aliases("zsh", &mut ZshVisitor {});
}
