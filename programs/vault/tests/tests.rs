use std::{assert_eq, path::PathBuf, println, vec};

use anchor_lang::{prelude::*, InstructionData};
use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_message::Instruction;
use solana_signer::Signer;
use solana_transaction::{Transaction, TransactionError};
use vault::Vault;

fn deployed_program_path(file_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/deploy")
        .join(file_name)
}

#[test]
fn test_try_transfer() {
    let mut svm = LiteSVM::new();
    svm.add_program_from_file(vault::ID, deployed_program_path("vault.so"))
        .unwrap();
    let payer = Keypair::new();
    let _ = svm.airdrop(&payer.pubkey(), 1_000_000_000);

    let (vault_pda, _) = Pubkey::find_program_address(&[b"vault"], &vault::ID);
    let instruction_accounts = vault::accounts::Initialize {
        vault: vault_pda,
        authority: payer.pubkey(),
        system_program: anchor_lang::system_program::ID,
    }
    .to_account_metas(None);

    let data = vault::instruction::Initialize.data();

    let instruction_initialize = Instruction {
        program_id: vault::ID,
        accounts: instruction_accounts,
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[instruction_initialize],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );
    let _res = svm.send_transaction(tx).unwrap();

    let vault = svm.get_account(&vault_pda).unwrap();

    assert_eq!(vault.owner, vault::ID);
    assert!(!vault.data.is_empty());
    assert!(vault.lamports > 0);

    let accounts = vault::accounts::TryTransfer {
        vault: vault_pda,
        recipient: payer.pubkey(),
        system_program: anchor_lang::system_program::ID,
    }
    .to_account_metas(None);

    let data = vault::instruction::TryTransfer { amount: 126 }.data();

    let instruction_try_transfer = Instruction {
        program_id: vault::ID,
        accounts,
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[instruction_try_transfer],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let err = svm.send_transaction(tx);

    println!("{:?}", err);
}

#[test]
fn test_uniq_pda() {
    let mut svm = LiteSVM::new();
    svm.add_program_from_file(vault::ID, deployed_program_path("vault.so"))
        .unwrap();
    let payer = Keypair::new();
    let _ = svm.airdrop(&payer.pubkey(), 1_000_000_000);

    let (pda, _) = Pubkey::find_program_address(&[b"vault clean"], &vault::ID);

    let _ = svm.airdrop(&pda, 1_000_000_000);

    let mut pda_account = svm.get_account(&pda).unwrap();
    pda_account.owner = vault::ID;
    pda_account.data = vec![];

    let _ = svm.set_account(pda, pda_account);

    let accounts = vault::accounts::TryTransferClean {
        vault: pda,
        recipient: payer.pubkey(),
        system_program: anchor_lang::system_program::ID,
    }
    .to_account_metas(None);

    let data = vault::instruction::TryTransferClean { amount: 125 }.data();

    let instructions = Instruction {
        accounts,
        program_id: vault::ID,
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[instructions],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );
    let err = svm.send_transaction(tx);

    println!("{:?}", err);
}

#[test]
fn test_check_rent() {
    let mut svm = LiteSVM::new();
    svm.add_program_from_file(vault::ID, deployed_program_path("vault.so"))
        .unwrap();
    let payer = Keypair::new();
    let _ = svm.airdrop(&payer.pubkey(), 1_000_000_000);

    let (pda, _) = Pubkey::find_program_address(&[b"vault"], &vault::ID);

    let accounts = vault::accounts::Initialize {
        vault: pda,
        authority: payer.pubkey(),
        system_program: anchor_lang::system_program::ID,
    }
    .to_account_metas(None);

    let data = vault::instruction::Initialize.data();

    let instruction = Instruction {
        program_id: vault::ID,
        accounts,
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );
    let _ = svm.send_transaction(tx);

    let account_vault = svm.get_account(&pda).unwrap();
    let mut vault_data = account_vault.data.as_slice();
    let vault = Vault::try_deserialize(&mut vault_data).unwrap();

    assert_eq!(vault.rent, 0);

    let accounts = vault::accounts::CheckRent { vault: pda }.to_account_metas(None);

    let data = vault::instruction::CheckRent.data();

    let instruction = Instruction {
        program_id: vault::ID,
        accounts,
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let _ = svm.send_transaction(tx);

    let vault_account = svm.get_account(&pda).unwrap();
    let mut vault_data = vault_account.data.as_slice();
    let vault = Vault::try_deserialize(&mut vault_data).unwrap();

    assert!(vault.rent > 0);

    let rent = svm.get_sysvar::<rent::Rent>();
    let data_len = Vault::DISCRIMINATOR.len() + Vault::INIT_SPACE;
    let min_rent = rent.minimum_balance(data_len);

    println!("{}", vault.rent);

    assert_eq!(vault.rent, min_rent);
}

#[test]
fn test_deposit() {
    let mut svm = LiteSVM::new();
    svm.add_program_from_file(vault::ID, deployed_program_path("vault.so"))
        .unwrap();

    let payer = Keypair::new();
    let _ = svm.airdrop(&payer.pubkey(), 1_000_000_000);

    let (pda, bump) = Pubkey::find_program_address(&[b"vault"], &vault::ID);

    let accounts = vault::accounts::Initialize {
        vault: pda,
        authority: payer.pubkey(),
        system_program: anchor_lang::system_program::ID,
    }
    .to_account_metas(None);

    let data = vault::instruction::Initialize.data();

    let instruction = Instruction {
        program_id: vault::ID,
        accounts,
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );
    let _ = svm.send_transaction(tx);

    let vault_account = svm.get_account(&pda).unwrap();

    let lamport_before_deposit = vault_account.lamports;

    let mut vault_data = vault_account.data.as_slice();
    let vault = Vault::try_deserialize(&mut vault_data).unwrap();

    assert_eq!(vault.amount, 0);
    assert_eq!(vault.authority, payer.pubkey());
    assert_eq!(vault.bump, bump);

    println!("lamports before deposit: {}", lamport_before_deposit);

    let accounts = vault::accounts::Deposit {
        vault: pda,
        authority: payer.pubkey(),
        system_program: anchor_lang::system_program::ID,
    }
    .to_account_metas(None);

    let data = vault::instruction::Deposit { amount: 125 }.data();

    let instruction = Instruction {
        program_id: vault::ID,
        accounts,
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );
    let _ = svm.send_transaction(tx);

    let vault_account = svm.get_account(&pda).unwrap();
    let mut vault_data = vault_account.data.as_slice();
    let vault = Vault::try_deserialize(&mut vault_data).unwrap();
    let lamports_after_deposit = vault_account.lamports;

    println!("lamports after deposit: {}", lamports_after_deposit);

    assert_eq!(lamports_after_deposit - lamport_before_deposit, 125);
    assert_eq!(vault.amount, 125);
    assert_eq!(vault.authority, payer.pubkey());
    assert_eq!(vault.bump, bump);
}

#[test]
fn test_deposit_amount_zero() {
    let mut svm = LiteSVM::new();
    svm.add_program_from_file(vault::ID, deployed_program_path("vault.so"))
        .unwrap();

    let payer = Keypair::new();
    let _ = svm.airdrop(&payer.pubkey(), 1_000_000_000);

    let (pda, _) = Pubkey::find_program_address(&[b"vault"], &vault::ID);

    let accounts = vault::accounts::Initialize {
        vault: pda,
        authority: payer.pubkey(),
        system_program: anchor_lang::system_program::ID,
    }
    .to_account_metas(None);

    let data = vault::instruction::Initialize.data();

    let instruction = Instruction {
        program_id: vault::ID,
        accounts,
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );
    let _ = svm.send_transaction(tx);

    let vault = svm.get_account(&pda).unwrap();

    let lamport_before_deposit = vault.lamports;

    let accounts = vault::accounts::Deposit {
        vault: pda,
        authority: payer.pubkey(),
        system_program: anchor_lang::system_program::ID,
    }
    .to_account_metas(None);

    let data = vault::instruction::Deposit { amount: 0 }.data();

    let instruction = Instruction {
        program_id: vault::ID,
        accounts,
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let err = svm.send_transaction(tx).unwrap_err();
    println!("{:?}", err);

    let vault = svm.get_account(&pda).unwrap();

    let vault_lamports_after = vault.lamports;

    assert_eq!(
        err.err,
        TransactionError::InstructionError(0, solana_transaction::InstructionError::Custom(6000))
    );
    assert_eq!(lamport_before_deposit, vault_lamports_after);
}
