use crate::model::{Alias, AliasConfig, AliasObjectValue};

pub fn run(config: AliasConfig, args: Vec<String>) {
    let alias_name = args.get(0).unwrap();
    let args_more = args.iter().skip(1).collect::<Vec<_>>();

    if let Some(alias) = config.get_alias(alias_name) {
        match alias {
            Alias::Inline(alias_args) => {
                let alias_args = shlex::split(alias_args).unwrap();
                let mut cmd = std::process::Command::new(alias_args.get(0).unwrap());
                let mut args = alias_args.iter().skip(1).collect::<Vec<_>>();
                args.extend(&args_more);
                cmd.args(args);
                cmd.spawn().unwrap().wait().unwrap();
            }
            Alias::Multi(alias_args) => {
                let mut cmd = std::process::Command::new(alias_args.get(0).unwrap());
                let mut args = alias_args.iter().skip(1).collect::<Vec<_>>();
                args.extend(&args_more);
                cmd.args(args);
                cmd.spawn().unwrap().wait().unwrap();
            }
            Alias::Object(platform_aliases) => {
                let platform =
                    std::env::var("ALIAS_SHELL").unwrap_or_else(|_| "default".to_string());
                let platform_alias_args = platform_aliases.get(&platform).expect("TODO:");

                let platform_alias_args = match platform_alias_args {
                    AliasObjectValue::Inline(alias_args) => shlex::split(alias_args).unwrap(),
                    AliasObjectValue::Multi(alias_args) => alias_args.clone(),
                };

                let mut cmd = std::process::Command::new(platform_alias_args.get(0).unwrap());
                let mut args = platform_alias_args.iter().skip(1).collect::<Vec<_>>();
                args.extend(&args_more);
                cmd.args(args);
                cmd.spawn().unwrap().wait().unwrap();
            }
        }
    }
}
