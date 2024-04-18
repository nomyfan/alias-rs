use super::bash::print_alias;
use crate::model::{AliasConfig, AliasVisitor, VisitorAliasValue};

const FISH_SCRIPT: &str = include_str!("./fish.fish");

struct FishVisitor {}

impl AliasVisitor for FishVisitor {
    fn visit<'a>(&mut self, (name, value): (&'a str, VisitorAliasValue<'a>)) {
        print_alias(name, value);
    }
}

pub fn init(config: AliasConfig) {
    println!("{FISH_SCRIPT}");

    config.visit_aliases("fish", &mut FishVisitor {});
}
