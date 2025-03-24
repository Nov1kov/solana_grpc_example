use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::system_instruction;

pub struct SendSolAction {
    transaction: Transaction,
}

impl SendSolAction {
    pub fn new(from_pubkey: &Pubkey, recipient_pubkey: &Pubkey, amount_lamports: u64) -> Self {
        let instruction = system_instruction::transfer(
            &from_pubkey,
            recipient_pubkey,
            amount_lamports,
        );

        let message = Message::new(&[instruction], Some(&from_pubkey));

        let mut transaction = Transaction::new_unsigned(message);
        Self { transaction }
    }

    pub fn execute(&self, blockhash: &str) -> anyhow::Result<()> {
        Ok(())
    }
}