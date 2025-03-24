use std::str::FromStr;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::hash::Hash;
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_sdk::system_instruction;
use tracing::*;

pub struct SendSolAction {
    transaction: Transaction,
    keypair: Keypair,
    rpc_client: RpcClient,
}

impl SendSolAction {
    pub fn new(keypair: Keypair, recipient_pubkey: &Pubkey, amount_lamports: u64, rpc_client: RpcClient) -> Self {
        let from_pubkey = &keypair.pubkey();

        Self {
            transaction: prepare_transaction(from_pubkey, recipient_pubkey, amount_lamports),
            keypair,
            rpc_client,
        }
    }

    pub async fn execute(&self, blockhash: &str) -> anyhow::Result<()> {
        let recent_blockhash: Hash = if
            let Ok(recent_blockhash) = self.rpc_client.get_latest_blockhash().await
        {
            recent_blockhash
        } else {
            Hash::from_str(blockhash)?
        };

        let mut transaction = self.transaction.clone();
        transaction.sign(&[&self.keypair], recent_blockhash);

        let txn_result = self.rpc_client.send_and_confirm_transaction(&transaction).await;
        if let Ok(txn_result) = txn_result {
            info!("tx sent: {:#?}", txn_result);
        } else {
            error!("send tx error: {:#?}", txn_result);
        }

        Ok(())
    }
}

fn prepare_transaction(
    from_pubkey: &Pubkey,
    recipient_pubkey: &Pubkey,
    amount_lamports: u64
) -> Transaction {
    let instruction = system_instruction::transfer(
        from_pubkey,
        recipient_pubkey,
        amount_lamports
    );
    let message = Message::new(&[instruction], Some(&from_pubkey));
    Transaction::new_unsigned(message)
}
