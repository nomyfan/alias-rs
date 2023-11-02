mod init;
mod model;
mod run;

use crate::model::AliasConfig;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    let config_path = dirs::home_dir().unwrap().join(".config").join("alias.toml");
    let config = std::fs::read_to_string(config_path).unwrap_or_default();
    let config = AliasConfig::from_str(&config).unwrap();

    let sub_command: &str = args.get(0).unwrap().as_ref();
    match sub_command {
        "run" => run::run(config, args.into_iter().skip(1).collect::<Vec<_>>()),
        "init" => init::init(config, args.into_iter().skip(1).collect::<Vec<_>>()),
        _ => unreachable!("Unknown command"),
    }
}
