mod app;
mod solana;
mod actions;

use crate::actions::executor::BotAction;
use crate::solana::geyser::{run_geyser_client_with_retry, BlockchainMessage};
use app::config::load_config;
use clap::{Parser, Subcommand};
use solana::wallet;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use std::str::FromStr;
use tokio::sync::mpsc;
use tracing::*;

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
    let keypair = Keypair::from_base58_string(&settings.wallet.private_key);
    let rpc_client = RpcClient::new(settings.solana_rpc.url.clone());

    tokio::spawn(async move {
        run_geyser_client_with_retry(settings_clone, tx.clone()).await;
    });

    if let Some(send_sol_action) = settings.actions.transfer_on_every_block {
        let action = BotAction::SendSol(actions::send_sol::SendSolAction::new(
            keypair,
            &Pubkey::from_str(&send_sol_action.recipient)?,
            send_sol_action.amount,
            rpc_client,
            settings.solana_rpc.is_prod,
        ));
        actions::executor::receiver(&mut rx, action).await;
    } else {
        return Err(anyhow::anyhow!("No action to execute"));
    }
    
    Ok(())
}
