use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::system_instruction;

struct SendSolAction {
    transaction: Transaction,
}

impl SendSolAction {
    pub fn new(recipient_pubkey: &Pubkey, amount_lamports: u64) -> Self {
        let from_pubkey = self.keypair.pubkey();
        let instruction = system_instruction::transfer(
            &from_pubkey,
            recipient_pubkey,
            amount_lamports,
        );

        // Создание сообщения транзакции
        let message = Message::new(&[instruction], Some(&from_pubkey));

        // Создание и подпись транзакции
        let mut transaction = Transaction::new_unsigned(message);
        Self { transaction }
    }
}