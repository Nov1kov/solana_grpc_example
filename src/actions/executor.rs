use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use tokio::task;
use tracing::*;
use crate::solana::geyser::BlockchainMessage;
use crate::actions::send_sol::SendSolAction;

const MAX_BLOCKS_IN_QUEUE: usize = 10;
const MAX_CONCURRENT_TASKS: usize = 100;

pub enum BotAction {
    SendSol(SendSolAction),
    // ...
}

pub async fn receiver(
    rx: &mut mpsc::Receiver<BlockchainMessage>,
    bot_action: BotAction,
) {
    let tx_action = Arc::new(bot_action);
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_TASKS));

    while let Some(message) = rx.recv().await {
        match message {
            BlockchainMessage::RecentBlockhash(blockhash) => {
                let blockhash_clone = blockhash.clone();
                let tx_action_clone = Arc::clone(&tx_action);
                let permit = semaphore.clone().acquire_owned().await.unwrap();

                let active_tasks = MAX_CONCURRENT_TASKS - semaphore.available_permits();
                if active_tasks > MAX_CONCURRENT_TASKS / 2 {
                    warn!("High task load: {} active tasks", active_tasks);
                }

                task::spawn(async move {
                    match &*tx_action_clone {
                        BotAction::SendSol(action) => {
                            if let Err(e) = action.execute(&blockhash_clone).await {
                                error!("Failed to execute SendSol action: {:?}", e);
                            }
                        }
                    }
                    drop(permit);
                });
            }
        }

        if rx.len() > MAX_BLOCKS_IN_QUEUE {
            warn!("{} blocks in queue", rx.len());
            continue;
        }
    }
}
