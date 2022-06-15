use jsonrpsee::core::Error;
use jsonrpsee::types::EmptyParams;
use jsonrpsee::ws_client::WsClientBuilder;

use crate::starknet::BlockHeader;
use crate::storage::components::storage_test_utils;

use super::api::*;
use super::*;

#[tokio::test]
async fn test_block_number() {
    let storage_components = storage_test_utils::get_test_storage();
    let storage_reader = storage_components.block_storage_reader;
    let mut storage_writer = storage_components.block_storage_writer;
    let module = JsonRpcServerImpl { storage_reader }.into_rpc();

    // No blocks yet.
    let err = module
        .call::<_, BlockNumber>("starknet_blockNumber", EmptyParams::new())
        .await
        .unwrap_err();
    let _expected = Error::from(api::JsonRpcError::NoBlocks);
    assert_matches!(err, _expected);

    // Add a block and check again.
    storage_writer
        .append_header(BlockNumber(0), &BlockHeader::default())
        .unwrap();
    let block_number = module
        .call::<_, BlockNumber>("starknet_blockNumber", EmptyParams::new())
        .await
        .unwrap();
    assert_eq!(block_number, BlockNumber(0));
}

#[tokio::test]
async fn test_run_server() {
    let storage_reader = storage_test_utils::get_test_storage().block_storage_reader;
    let (addr, _handle) = run_server(storage_reader).await.unwrap();
    let client = WsClientBuilder::default()
        .build(format!("ws://{:?}", addr))
        .await
        .unwrap();
    let err = client.block_number().await.unwrap_err();
    let _expected = Error::from(api::JsonRpcError::NoBlocks);
    assert_matches!(err, _expected);
}