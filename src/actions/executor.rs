use tokio::sync::mpsc;
use crate::solana::geyser::BlockchainMessage;

pub async fn receiver(rx: &mut mpsc::Receiver<BlockchainMessage>) {
    while let Ok(message) = rx.recv().await {
        match message {
            BlockchainMessage::RecentBlockhash(blockhash) => {
                println!("Received recent blockhash: {}", blockhash);
            }
        }
    }
}
