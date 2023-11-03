use crate::model::{AliasConfig, AliasVisitor, VisitorAliasValue};

const PWSH_SCRIPT: &str = include_str!("./pwsh.ps1");

struct PwshVisitor {}

impl AliasVisitor for PwshVisitor {
    fn visit<'a>(&mut self, (fn_name, fn_body): (&'a str, VisitorAliasValue<'a>)) {
        println!("function {fn_name} {{ {fn_body} }}");
    }
}

pub fn init(config: AliasConfig) {
    println!("{PWSH_SCRIPT}");

    config.visit_aliases("powershell", PwshVisitor {});
}
