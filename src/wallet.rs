use std::str::FromStr;
use anyhow::Result;
use solana_sdk::hash::Hash;
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use tracing::info;
use base64::{engine::general_purpose, Engine as _};

pub fn create_wallet() -> Result<()> {
    let keypair = Keypair::new();
    let pubkey: Pubkey = keypair.pubkey();
    let private_key_bs58 = bs58::encode(keypair.to_bytes()).into_string();

    info!("Created new Solana wallet");
    info!("\tpublic_key: {}", pubkey);
    info!("\tprivate_key: {}", private_key_bs58);

    Ok(())
}

pub struct Wallet {
    keypair: Keypair,
}

impl Wallet {
    pub fn new(private_key_bs58: &str) -> Self {
        Wallet {
            keypair: Keypair::from_base58_string(private_key_bs58),
        }
    }

    pub fn sign_sol_transfer(
        &self,
        recipient_pubkey: &Pubkey,
        amount_lamports: u64,
        recent_blockhash: &str
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Создание инструкции для перевода SOL
        let instruction = system_instruction::transfer(
            &self.keypair.pubkey(),
            recipient_pubkey,
            amount_lamports,
        );

        // Создание сообщения транзакции
        let message = Message::new(&[instruction], Some(&self.keypair.pubkey()));

        // Создание и подпись транзакции
        let mut transaction = Transaction::new_unsigned(message);
        transaction.sign(&[&self.keypair], Hash::from_str(recent_blockhash).unwrap());

        // Сериализация и кодирование транзакции в base64
        let serialized_transaction = bincode::serialize(&transaction)?;
        let encoded_transaction = general_purpose::STANDARD.encode(serialized_transaction);

        Ok(encoded_transaction)
    }
}