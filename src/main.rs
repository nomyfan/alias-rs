mod init;
mod model;

use crate::model::AliasConfig;

fn main() -> anyhow::Result<()> {
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

    let sub_command = args
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("Usage: als init <shell>"))?;
    match sub_command.as_ref() {
        "init" => {
            init::init(config, args.into_iter().skip(1).collect::<Vec<_>>());
        }
        _ => anyhow::bail!("Unknown command"),
    }

    Ok(())
}
