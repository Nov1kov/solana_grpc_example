mod app;
mod solana;
mod actions;

use std::str::FromStr;
use std::time::Duration;
use clap::{Parser, Subcommand};
use solana_client::nonblocking::rpc_client::RpcClient;
use tracing::*;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient};
use app::config::load_config;
use solana::{geyser, shyft_api, wallet};

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

    match geyser::get_client(&settings.shyft_grpc).await {
        Ok(client) => {
            let wallet = wallet::Wallet::new(&settings.wallet.private_key);

            let request = geyser::get_block_subscribe_request();

            let rpc_client = RpcClient::new(settings.solana_rpc.url);
        
            geyser::geyser_subscribe(client, request, &wallet, &rpc_client).await?;
        },
        Err(e) => {
            // sleep for recoonect
            tokio::time::sleep(Duration::from_secs(10)).await;
            warn!("Failed to create Geyser client: {}", e);
        }
    };

    Ok(())
}
