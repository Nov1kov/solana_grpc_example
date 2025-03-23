use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use tracing::info;

pub fn create_wallet() -> Result<()> {
    let keypair = Keypair::new();
    let pubkey: Pubkey = keypair.pubkey();
    let private_key_bs58 = bs58::encode(keypair.to_bytes()).into_string();

    info!("Created new Solana wallet");
    info!("\tpublic_key: {}", pubkey);
    info!("\tprivate_key: {}", private_key_bs58);

    Ok(())
}