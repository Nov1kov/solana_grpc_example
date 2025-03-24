use std::collections::HashMap;
use std::str::FromStr;
use std::time::{ Duration, SystemTime, UNIX_EPOCH };
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use yellowstone_grpc_client::{ ClientTlsConfig, GeyserGrpcClient, Interceptor };
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel,
    SubscribeRequest,
    SubscribeRequestFilterBlocks,
};
use yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof;
use yellowstone_grpc_proto::tonic::codegen::tokio_stream::StreamExt;
use tracing::*;
use crate::app::config::{ Settings, ShyftGrpcConfig };
use crate::solana::shyft_api::ShyftClient;
use crate::solana::wallet::Wallet;

pub fn get_block_subscribe_request() -> SubscribeRequest {
    let mut blocks = HashMap::new();
    blocks.insert("client".to_owned(), SubscribeRequestFilterBlocks {
        account_include: vec![],
        include_transactions: None,
        include_accounts: None,
        include_entries: None,
    });

    SubscribeRequest {
        slots: HashMap::default(),
        accounts: HashMap::default(),
        transactions: HashMap::default(),
        transactions_status: HashMap::default(),
        entry: HashMap::default(),
        blocks: blocks,
        blocks_meta: HashMap::default(),
        commitment: Some(CommitmentLevel::Processed as i32),
        accounts_data_slice: Vec::default(),
        ping: None,
        from_slot: None,
    }
}

pub async fn geyser_subscribe(
    mut _client: GeyserGrpcClient<impl Interceptor>,
    request: SubscribeRequest,
    wallet: &Wallet,
    shyft_client: &ShyftClient,
    rpc_client: &RpcClient
) -> anyhow::Result<()> {
    let (mut subscribe_tx, mut stream) = _client.subscribe_with_request(Some(request)).await?;

    info!("stream opened");
    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => {
                match msg.update_oneof {
                    Some(UpdateOneof::Block(msg)) => {
                        let slot: &u64 = &msg.slot;
                        let slot_cloned = slot.clone();
                        let block_time = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards")
                            .as_secs() as i64;
                        let recent_blockhash: &str = if
                            let Ok(recent_blockhash) = rpc_client.get_latest_blockhash().await
                        {
                            println!("Devnet recent_blockhash: {:?}", recent_blockhash);
                            &recent_blockhash.to_string()
                        } else {
                            &msg.blockhash
                        };
                        if
                            let Ok(pubkey) = Pubkey::from_str(
                                "DSUby69eVtXoDnmaQ4qQQtS5fJeE2omXWBA2qCxe8yTg"
                            )
                        {
                            if true {
                                let tx = wallet.get_signed_transaction(
                                    &pubkey,
                                    1000000,
                                    recent_blockhash
                                );
                                println!("tx: {:?}", tx);
                                if let Ok(tx) = tx {
                                    let txn_result = rpc_client.send_and_confirm_transaction(
                                        &tx
                                    ).await;
                                    if let Ok(txn_result) = txn_result {
                                        info!("tx sent: {:#?}", txn_result);
                                    } else {
                                        error!("send tx error: {:#?}", txn_result);
                                    }
                                }
                            } else {
                                if
                                    let Ok(tx) = wallet.sign_sol_transfer(
                                        &pubkey,
                                        100000,
                                        recent_blockhash
                                    )
                                {
                                    println!("tx: {}", tx);
                                    let txn_result = shyft_client.send_transaction(tx).await;
                                    if let Ok(txn_result) = txn_result {
                                        info!("tx sent: {:#?}", txn_result);
                                    } else {
                                        error!("error: {:#?}", txn_result);
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        error!("update not found in the message");
                        break;
                    }
                    _ => {}
                }
            }
            Err(error) => {
                error!("error: {error:?}");
                break;
            }
        }
    }

    info!("stream closed");
    Ok(())
}

pub async fn get_client(
    shyft_grpc_config: &ShyftGrpcConfig
) -> Result<GeyserGrpcClient<impl Interceptor>, anyhow::Error> {
    let client = GeyserGrpcClient::build_from_shared(shyft_grpc_config.url.clone())?
        .x_token(Some(&shyft_grpc_config.x_token))?
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(10))
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .max_decoding_message_size(1024 * 1024 * 1024)
        .connect().await?;

    info!("Connecting to endpoint: {}", shyft_grpc_config.url);

    Ok(client)
}
