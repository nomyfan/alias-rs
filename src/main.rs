mod init;
mod model;

use crate::model::AliasConfig;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    let config_path = dirs::home_dir().unwrap().join(".config").join("alias.toml");
    let config = std::fs::read_to_string(config_path).unwrap_or_else(|_| "[aliases]".to_string());
    let config = match AliasConfig::from_str(&config) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to parse config file {e}");
            AliasConfig::default()
        }
    };

    let sub_command: &str = args.get(0).unwrap().as_ref();
    match sub_command {
        "init" => init::init(config, args.into_iter().skip(1).collect::<Vec<_>>()),
        _ => unreachable!("Unknown command"),
    }
}
