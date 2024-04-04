use crate::model::{AliasConfig, AliasVisitor, VisitorAliasValue};

const BASH_SCRIPT: &str = include_str!("./bash.bash");

#[inline]
pub fn print_alias<V: std::fmt::Display>(name: &str, value: V) {
    println!("alias {}=\"{}\"", name, value)
}

struct BashVisitor {}

impl AliasVisitor for BashVisitor {
    fn visit<'a>(&mut self, (name, value): (&'a str, VisitorAliasValue<'a>)) {
        print_alias(name, value);
    }
}

pub fn init(config: AliasConfig) {
    println!("{BASH_SCRIPT}");

    config.visit_aliases("bash", &mut BashVisitor {});
}
