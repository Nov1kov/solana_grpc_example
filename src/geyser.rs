use std::collections::HashMap;
use std::time::{ SystemTime, UNIX_EPOCH };
use yellowstone_grpc_client::{ GeyserGrpcClient, Interceptor };
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel,
    SubscribeRequest,
    SubscribeRequestFilterBlocks,
};
use yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof;
use yellowstone_grpc_proto::tonic::codegen::tokio_stream::StreamExt;
use tracing::*;

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
    request: SubscribeRequest
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
                        let block_hash: &str = &msg.blockhash;
                        let block_time = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards")
                            .as_secs() as i64;
                        let transactions: Vec<yellowstone_grpc_proto::prelude::SubscribeUpdateTransactionInfo> =
                            msg.transactions;
                        info!("slot: {}, block time: {}, block: {}", slot, block_time, block_hash);
                    }
                    Some(UpdateOneof::Ping(_)) => {
                        info!("ping");
                    }
                    Some(UpdateOneof::Pong(_)) => {
                        // Handle pong response if needed
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