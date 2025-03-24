mod config;
mod geyser;
mod wallet;
mod shyft_api;

use std::str::FromStr;
use std::time::Duration;
use clap::{Parser, Subcommand};
use tracing::*;
use yellowstone_grpc_client::{ ClientTlsConfig, GeyserGrpcClient };
use crate::config::load_config;


#[derive(Parser, Debug)]
#[command(name = "Solana tasker")]
#[command(about = "Automate tasks in Solana", long_about = None)]
struct Args {
    #[arg(short, long, value_name = "CONFIG_FILE", default_value = "config.yaml")]
    config_file: String,

    #[arg(short, long, value_name = "LOG_LEVEL", default_value = "INFO")]
    log_level: String,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate a new Solana wallet (keypair)
    CreateWallet,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    tracing_subscriber
        ::fmt()
        .with_target(false)
        .compact()
        .with_max_level(Level::from_str(&args.log_level).unwrap())
        .init();


    match args.command {
        Some(Command::CreateWallet) => {
            // Generate and display wallet
            wallet::create_wallet()?;
            return Ok(());
        }
        None => {
            // Default behavior - continue with the original program
        }
    }

    let settings = load_config(&args.config_file);

    info!("Connecting to endpoint: {}", settings.shyft_grpc.url);

    let client = GeyserGrpcClient::build_from_shared(settings.shyft_grpc.url)
        .unwrap()
        .x_token(Some(&settings.shyft_grpc.x_token))
        .unwrap()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(10))
        .tls_config(ClientTlsConfig::new().with_native_roots())
        .unwrap()
        .max_decoding_message_size(1024 * 1024 * 1024)
        .connect().await
        .unwrap();

    let wallet = wallet::Wallet::new(&settings.wallet.private_key);

    let request = geyser::get_block_subscribe_request();

    let shyft_client = shyft_api::ShyftClient::new(&settings.shyft_rpc.url, &settings.shyft_rpc.api_key, &settings.shyft_rpc.network);

    geyser::geyser_subscribe(client, request, &wallet, &shyft_client).await?;

    Ok(())
}
