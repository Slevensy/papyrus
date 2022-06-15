use std::time::Duration;

use async_stream::stream;
use log::info;
use tokio_stream::Stream;

use crate::starknet::{BlockHeader, BlockNumber};
use crate::starknet_client::{ClientError, StarknetClient};

pub struct CentralSource {
    starknet_client: StarknetClient,
}

// TODO(spapini): Take from config.
const STARKNET_URL: &str = "https://alpha4.starknet.io/";
const SLEEP_DURATION: Duration = Duration::from_millis(10000);

impl CentralSource {
    pub fn new() -> Result<CentralSource, ClientError> {
        let starknet_client = StarknetClient::new(STARKNET_URL)?;
        Ok(CentralSource { starknet_client })
    }

    pub async fn get_block_number(&mut self) -> Result<BlockNumber, ClientError> {
        self.starknet_client.block_number().await
    }

    // TODO(spapini): Return blocks instead of numbers.
    pub fn stream_new_blocks(
        &mut self,
        initial_block_number: BlockNumber,
    ) -> impl Stream<Item = (BlockNumber, BlockHeader)> + '_ {
        let mut current_block_number = initial_block_number;
        stream! {
            while let Ok(BlockNumber(latest_block_number)) = self.get_block_number().await {
                while current_block_number.0 <= latest_block_number {
                    info!("Received new block number: {}.", current_block_number.0);
                    yield (current_block_number, BlockHeader::default());
                    current_block_number = current_block_number.next();
                }
                tokio::time::sleep(SLEEP_DURATION).await
            }
        }
    }
}