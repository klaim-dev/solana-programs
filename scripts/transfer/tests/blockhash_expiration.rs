use std::{thread, time::Duration};

use anyhow::{Context, Result};
use solana_rpc_client::rpc_client::RpcClient;

const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";

#[test]
#[ignore = "uses live devnet and waits for the blockhash to expire"]
fn blockhash_becomes_invalid_after_last_valid_block_height() -> Result<()> {
    let rpc_client = RpcClient::new(DEVNET_RPC_URL);
    let commitment = rpc_client.commitment();
    let (blockhash, last_valid_block_height) = rpc_client
        .get_latest_blockhash_with_commitment(commitment)
        .context("failed to fetch recent blockhash")?;

    while rpc_client
        .get_block_height()
        .context("failed to fetch current block height")?
        <= last_valid_block_height
    {
        thread::sleep(Duration::from_millis(400));
    }

    let is_valid = rpc_client
        .is_blockhash_valid(&blockhash, commitment)
        .context("failed to check blockhash validity")?;

    assert!(
        !is_valid,
        "blockhash remained valid past its last valid height"
    );
    Ok(())
}
