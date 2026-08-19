use std::{
    io::{self, Write},
    str::FromStr,
};

use anyhow::{Context, Result, bail};
use solana_keypair::read_keypair_file;
use solana_pubkey::Pubkey;
use solana_rpc_client::rpc_client::RpcClient;
use solana_signer::Signer;
use solana_system_interface::instruction::transfer;
use solana_transaction::Transaction;

const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";
const PAYER_KEYPAIR_PATH: &str = "/home/klaim/.config/solana/id.json";
const RECIPIENT: &str = "HgafDv195BtNc8X4uvNoRuGcUra5PuUwDJgHeKHvgFiS";
const LAMPORTS: u64 = 1_000_000;

fn main() -> Result<()> {
    let rpc_client = RpcClient::new(DEVNET_RPC_URL);
    let payer = read_keypair_file(PAYER_KEYPAIR_PATH)
        .map_err(|error| anyhow::anyhow!("failed to read payer keypair: {error}"))?;
    let payer_pubkey = payer.pubkey();
    let recipient = Pubkey::from_str(RECIPIENT).context("failed to parse recipient pubkey")?;

    print_transfer_summary(&payer_pubkey, &recipient);
    if !confirm_transfer()? {
        println!("Transfer cancelled");
        return Ok(());
    }

    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .context("failed to fetch recent blockhash")?;
    let instruction = transfer(&payer_pubkey, &recipient, LAMPORTS);
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer_pubkey),
        &[&payer],
        recent_blockhash,
    );

    let simulation = rpc_client
        .simulate_transaction(&transaction)
        .context("failed to simulate transaction")?;
    if let Some(error) = simulation.value.err {
        bail!("transaction simulation failed: {error:?}");
    }

    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner(&transaction)
        .context("failed to send or confirm transaction")?;
    println!("Confirmed transaction: {signature}");

    Ok(())
}

fn print_transfer_summary(payer: &Pubkey, recipient: &Pubkey) {
    println!("Cluster: devnet");
    println!("Fee payer: {payer}");
    println!("Recipient: {recipient}");
    println!("Amount: {LAMPORTS} lamports");
}

fn confirm_transfer() -> Result<bool> {
    print!("Send this transaction? [y/N]: ");
    io::stdout().flush().context("failed to flush stdout")?;

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .context("failed to read confirmation")?;

    Ok(matches!(
        answer.trim().to_ascii_lowercase().as_str(),
        "y" | "yes"
    ))
}
