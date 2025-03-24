use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task;
use crate::solana::geyser::BlockchainMessage;
use tracing::*;
use crate::actions::send_sol::SendSolAction;

const MAX_BLOCKS_IN_QUEUE: usize = 10;

pub enum TransactionAction {
    SendSol(SendSolAction),
    // ...
}

pub async fn receiver(
    rx: &mut mpsc::Receiver<BlockchainMessage>,
    tx_action: TransactionAction,
) {
    let tx_action = Arc::new(tx_action);
    while let Some(message) = rx.recv().await {
        match message {
            BlockchainMessage::RecentBlockhash(blockhash) => {
                let blockhash_clone = blockhash.clone();
                let tx_action_clone = Arc::clone(&tx_action);

                task::spawn(async move {
                    match &*tx_action_clone {
                        TransactionAction::SendSol(action) => {
                            if let Err(e) = action.execute(&blockhash_clone).await {
                                error!("Failed to execute SendSol action: {:?}", e);
                            }
                        }
                    }
                });
            }
        }
        if rx.len() > MAX_BLOCKS_IN_QUEUE {
            warn!("{} blocks in queue", rx.len());
            continue;
        }
    }
}