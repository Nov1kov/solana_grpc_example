use std::path::PathBuf;
use serde::Deserialize;
use config::{ Config, ConfigError, Source };

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub shyft_grpc: ShyftGrpcConfig,
    pub wallet: WalletConfig,
    pub solana_rpc: SolanaRpcConfig,
    pub actions: Actions,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShyftGrpcConfig {
    pub url: String,
    pub x_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletConfig {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SolanaRpcConfig {
    pub url: String,
    pub is_prod: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Actions {
    #[serde(default)]
    pub transfer_on_every_block: Option<TransferAction>,
    // Можно добавить другие типы действий здесь
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransferAction {
    pub recipient: String,
    pub amount: u64,
}

fn parse_config<S>(content: S) -> Result<Settings, ConfigError>
    where S: Source + Send + Sync + 'static
{
    let config = Config::builder().add_source(content).build()?;
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
        let yaml_content =
            r#"
shyft_grpc:
  url: "https://test.grpc.shyft.to"
  x_token: "test_token_12345"

wallet:
  private_key: "test_private_key_abc123"
  public_key: "test_public_key_xyz789"

solana_rpc:
  url: "https://devnet-rpc.shyft.to?api_key=12345"
  is_prod: false

actions:
  transfer_on_every_block:
    recipient: "DSUby69eVtXoDnmaQ4qQQtS5fJeE2omXWBA2qCxe8yTg"
    amount: 100000
"#;

        let content = config::File::from_str(yaml_content, FileFormat::Yaml);
        let settings: Settings = parse_config(content).unwrap();

        assert_eq!(settings.shyft_grpc.x_token, "test_token_12345");
        assert_eq!(settings.shyft_grpc.url, "https://test.grpc.shyft.to");
        assert_eq!(settings.wallet.private_key, "test_private_key_abc123");
        assert_eq!(settings.wallet.public_key, "test_public_key_xyz789");
        assert_eq!(settings.solana_rpc.url, "https://devnet-rpc.shyft.to?api_key=12345");
        assert_eq!(settings.solana_rpc.is_prod, false);
        let action = settings.actions.transfer_on_every_block.unwrap();
        assert_eq!(action.recipient, "DSUby69eVtXoDnmaQ4qQQtS5fJeE2omXWBA2qCxe8yTg");
        assert_eq!(action.amount, 100000);
    }
}
