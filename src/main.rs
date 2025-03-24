mod app;
mod solana;
mod actions;

use std::str::FromStr;
use std::time::Duration;
use clap::{Parser, Subcommand};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use tokio::sync::mpsc;
use tracing::*;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient};
use app::config::load_config;
use solana::{geyser, wallet};
use crate::actions::executor::TransactionAction;
use crate::app::config::Settings;
use crate::solana::geyser::BlockchainMessage;

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
        .with_max_level(Level::from_str(&args.log_level)?)
        .init();

    match args.command {
        Some(Command::CreateWallet) => {
            // Generate and display wallet
            return wallet::create_wallet();
        }
        None => {
            // Default behavior
        }
    }

    let settings = load_config(&args.config_file);
    let settings_clone = settings.clone();
    let (tx, mut rx) = mpsc::channel::<BlockchainMessage>(100);
    let keypair: Keypair = Keypair::from_base58_string(&settings.wallet.private_key);

    tokio::spawn(async move {
        run_geyser_client_with_retry(settings_clone, tx.clone()).await;
    });

    let mut actions = vec![];
    if let Some(send_sol_action) = settings.actions.transfer_on_every_block {
        actions.push(TransactionAction::SendSol(actions::send_sol::SendSolAction::new(
            &keypair.pubkey(),
            &Pubkey::from_str(&send_sol_action.recipient)?,
            send_sol_action.amount,
        )));
    }

    actions::executor::receiver(&mut rx, actions).await;
    Ok(())
}

async fn run_geyser_client_with_retry(settings: Settings, tx: mpsc::Sender<BlockchainMessage>) {
    const RETRY_DELAY: u64 = 10; // Retry delay in seconds
    let request = geyser::get_block_subscribe_request();

    loop {
        match geyser::get_client(&settings.shyft_grpc).await {
            Ok(client) => {
                match geyser::geyser_subscribe(client, request.clone(), tx.clone()).await {
                    Ok(_) => {
                        info!("Subscribed");
                    },
                    Err(e) => {
                        warn!("Subscription error: {}", e);
                    }
                }
            },
            Err(e) => {
                warn!("Failed to create Geyser client: {}", e);
            }
        };

        info!("Reconnecting in {} seconds...", RETRY_DELAY);
        tokio::time::sleep(Duration::from_secs(RETRY_DELAY)).await;
    }
}