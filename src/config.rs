use std::path::PathBuf;
use serde::Deserialize;
use config::{Config, ConfigError, Source};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub x_token: String,
    pub endpoint: String,
    pub wallet: WalletConfig,
}

#[derive(Debug, Deserialize)]
pub struct WalletConfig {
    pub private_key: String,
    pub public_key: String,
}

fn parse_config<S>(content: S) -> Result<Settings, ConfigError>
where
    S: Source + Send + Sync + 'static,
{
    let config = Config::builder()
        .add_source(content)
        .build()?;
    config.try_deserialize()
}

pub fn load_config(file_name: &str) -> Settings {
    let config_path = PathBuf::from(file_name);
    if !config_path.exists() {
        panic!("Configuration file not found: {:?}", config_path);
    }
    let content = config::File::from(config_path);
    match parse_config(content) {
        Ok(settings) => settings,
        Err(e) => {
            panic!("Failed to parse configuration: {}", e);
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use config::FileFormat;

    #[test]
    fn test_load_config() {
        let yaml_content = r#"
x_token: "test_token_12345"
endpoint: "https://test.grpc.shyft.to"
wallet:
  private_key: "test_private_key_abc123"
  public_key: "test_public_key_xyz789"
"#;

        let content = config::File::from_str(yaml_content, FileFormat::Yaml);
        let settings: Settings = parse_config(content).unwrap();

        assert_eq!(settings.x_token, "test_token_12345");
        assert_eq!(settings.endpoint, "https://test.grpc.shyft.to");
        assert_eq!(settings.wallet.private_key, "test_private_key_abc123");
        assert_eq!(settings.wallet.public_key, "test_public_key_xyz789");
    }
}