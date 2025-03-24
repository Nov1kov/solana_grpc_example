use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient, Interceptor };
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel,
    SubscribeRequest,
    SubscribeRequestFilterBlocks,
};
use yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof;
use yellowstone_grpc_proto::tonic::codegen::tokio_stream::StreamExt;
use tracing::*;
use crate::app::config::{Settings, ShyftGrpcConfig};
use crate::solana::geyser;

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

#[derive(Debug, Clone)]
pub enum BlockchainMessage {
    RecentBlockhash(String),
}

pub async fn geyser_subscribe(
    mut client: GeyserGrpcClient<impl Interceptor>,
    request: SubscribeRequest,
    tx: mpsc::Sender<BlockchainMessage>
) -> anyhow::Result<()> {
    let (_, mut stream) = client.subscribe_with_request(Some(request)).await?;
    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => {
                match msg.update_oneof {
                    Some(UpdateOneof::Block(msg)) => {
                        tx.send(BlockchainMessage::RecentBlockhash(msg.blockhash)).await?;
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

pub async fn run_geyser_client_with_retry(settings: Settings, tx: mpsc::Sender<BlockchainMessage>) {
    const RETRY_DELAY: u64 = 10; // Retry delay in seconds
    let request = get_block_subscribe_request();

    loop {
        match get_client(&settings.shyft_grpc).await {
            Ok(client) => {
                match geyser::geyser_subscribe(client, request.clone(), tx.clone()).await {
                    Ok(_) => {
                        info!("Subscribed");
                    },
                    Err(e) => {
                        warn!("Subscription error: {}", e);
                    }
                }
            },
            Err(e) => {
                warn!("Failed to create Geyser client: {}", e);
            }
        };

        info!("Reconnecting in {} seconds...", RETRY_DELAY);
        tokio::time::sleep(Duration::from_secs(RETRY_DELAY)).await;
    }
}