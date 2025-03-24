use std::str::FromStr;
use anyhow::Result;
use solana_sdk::hash::Hash;
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::{system_instruction, system_transaction};
use solana_sdk::transaction::Transaction;
use tracing::info;
use base64::{engine::general_purpose, Engine as _};
use base64::prelude::BASE64_STANDARD;

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

    pub fn sign_transaction(
        &self,
        transaction: Transaction,
        recent_blockhash: Hash
    ) -> Transaction {
        let mut transaction = transaction;
        transaction.sign(&[&self.keypair], recent_blockhash);
        transaction
    }

    pub fn get_signed_transaction(
        &self,
        recipient_pubkey: &Pubkey,
        amount_lamports: u64,
        recent_blockhash: &str
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
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
        // println!("recent_blockhash str: {}", recent_blockhash);
        let recent_blockhash = Hash::from_str(recent_blockhash).unwrap();
        // println!("recent_blockhash hash: {:?}", recent_blockhash);
        transaction.sign(&[&self.keypair], recent_blockhash);

        Ok(transaction)
    }
}