use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_system_interface::instruction::transfer;
use solana_transaction::Transaction;

#[test]
fn same_message_produces_same_signature() {
    let payer = Keypair::new();
    let recipient = Pubkey::new_unique();
    let instruction = transfer(&payer.pubkey(), &recipient, 1_000_000);
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        Default::default(),
    );
    let message_bytes = transaction.message.serialize();

    let first_signature = payer.sign_message(&message_bytes);
    let second_signature = payer.sign_message(&message_bytes);

    assert_eq!(first_signature, second_signature);
}
