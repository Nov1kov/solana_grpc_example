use tokio::sync::mpsc;
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
    actions: Vec<TransactionAction>
) {
    while let Some(message) = rx.recv().await {
        match message {
            BlockchainMessage::RecentBlockhash(blockhash) => {
                for action in actions.iter() {
                    match action {
                        TransactionAction::SendSol(action) => {
                            action.execute(&blockhash);
                        }
                    }
                }
            }
        }

        if rx.len() > MAX_BLOCKS_IN_QUEUE {
            warn!("{} blocks in queue", rx.len());
            continue;
        }
    }
}
