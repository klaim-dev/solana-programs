use std::{path::PathBuf, str::FromStr};

use anchor_lang::{prelude::Pubkey, AccountDeserialize, InstructionData, ToAccountMetas};
use counter::state::Counter;
use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_message::{AccountMeta, Instruction};
use solana_signer::Signer;
use solana_transaction::{InstructionError, Transaction, TransactionError};

const NATIVE_PROGRAM_ADDRESS: &str = "AHPJLLL4YaEZyRaEUdEAvLWYC2p6XbrUW4j5gNFTtJUK";

fn deployed_program_path(file_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/deploy")
        .join(file_name)
}

fn read_anchor_counter(svm: &LiteSVM, address: &Pubkey) -> Counter {
    let account = svm
        .get_account(address)
        .expect("Anchor counter account should exist");
    Counter::try_deserialize(&mut account.data.as_slice())
        .expect("Anchor counter account should contain valid data")
}

fn read_native_counter(svm: &LiteSVM, address: &Pubkey) -> u64 {
    let account = svm
        .get_account(address)
        .expect("native state account should exist");
    let bytes = account.data[..8]
        .try_into()
        .expect("native state account should contain at least eight bytes");
    u64::from_le_bytes(bytes)
}

#[test]
fn initializes_and_increments_counter() {
    let mut svm = LiteSVM::new();
    svm.add_program_from_file(counter::ID, deployed_program_path("counter.so"))
        .expect("counter program should load");

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000)
        .expect("payer airdrop should succeed");
    let counter_account = Keypair::new();

    let initialize = Instruction {
        program_id: counter::ID,
        accounts: counter::accounts::Initialize {
            counter: counter_account.pubkey(),
            authority: payer.pubkey(),
            system_program: anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
        data: counter::instruction::Initialize.data(),
    };
    let initialize_transaction = Transaction::new_signed_with_payer(
        &[initialize],
        Some(&payer.pubkey()),
        &[&payer, &counter_account],
        svm.latest_blockhash(),
    );

    svm.send_transaction(initialize_transaction)
        .expect("counter initialization should succeed");
    assert_eq!(
        read_anchor_counter(&svm, &counter_account.pubkey()).value,
        0
    );

    let increment = Instruction {
        program_id: counter::ID,
        accounts: counter::accounts::Increment {
            counter: counter_account.pubkey(),
        }
        .to_account_metas(None),
        data: counter::instruction::Increment.data(),
    };

    for _ in 0..3 {
        let transaction = Transaction::new_signed_with_payer(
            &[increment.clone()],
            Some(&payer.pubkey()),
            &[&payer],
            svm.latest_blockhash(),
        );
        svm.send_transaction(transaction)
            .expect("counter increment should succeed");
        svm.expire_blockhash();
    }

    assert_eq!(
        read_anchor_counter(&svm, &counter_account.pubkey()).value,
        3
    );
}

#[test]
fn compares_native_and_anchor_increment_compute_units() {
    let mut svm = LiteSVM::new();
    let native_program_id =
        Pubkey::from_str(NATIVE_PROGRAM_ADDRESS).expect("native program ID should be valid");

    svm.add_program_from_file(native_program_id, deployed_program_path("hello_solana.so"))
        .expect("native program should load");
    svm.add_program_from_file(counter::ID, deployed_program_path("counter.so"))
        .expect("Anchor program should load");

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000)
        .expect("payer airdrop should succeed");

    let anchor_counter = Keypair::new();
    let initialize_anchor_counter = Instruction {
        program_id: counter::ID,
        accounts: counter::accounts::Initialize {
            counter: anchor_counter.pubkey(),
            authority: payer.pubkey(),
            system_program: anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
        data: counter::instruction::Initialize.data(),
    };
    let initialize_transaction = Transaction::new_signed_with_payer(
        &[initialize_anchor_counter],
        Some(&payer.pubkey()),
        &[&payer, &anchor_counter],
        svm.latest_blockhash(),
    );
    svm.send_transaction(initialize_transaction)
        .expect("Anchor counter initialization should succeed");

    let anchor_account_size = svm
        .get_account(&anchor_counter.pubkey())
        .expect("Anchor counter account should exist after initialization")
        .data
        .len();
    assert_eq!(read_anchor_counter(&svm, &anchor_counter.pubkey()).value, 0);

    let native_state = Keypair::new();
    svm.airdrop(&native_state.pubkey(), 1_000_000_000)
        .expect("native state airdrop should succeed");
    let mut native_state_account = svm
        .get_account(&native_state.pubkey())
        .expect("native state account should exist after airdrop");
    native_state_account.data = vec![0; anchor_account_size];
    native_state_account.owner = native_program_id;
    native_state_account.executable = false;
    svm.set_account(native_state.pubkey(), native_state_account)
        .expect("native state account should be configured");

    let actual_native_account_size = svm
        .get_account(&native_state.pubkey())
        .expect("configured native state account should exist")
        .data
        .len();
    assert_eq!(actual_native_account_size, anchor_account_size);
    assert_eq!(read_native_counter(&svm, &native_state.pubkey()), 0);

    let native_increment = Instruction {
        program_id: native_program_id,
        accounts: vec![AccountMeta::new(native_state.pubkey(), false)],
        data: vec![0],
    };
    let native_increment_transaction = Transaction::new_signed_with_payer(
        &[native_increment],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let anchor_increment = Instruction {
        program_id: counter::ID,
        accounts: counter::accounts::Increment {
            counter: anchor_counter.pubkey(),
        }
        .to_account_metas(None),
        data: counter::instruction::Increment.data(),
    };
    let anchor_increment_transaction = Transaction::new_signed_with_payer(
        &[anchor_increment],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let native_simulation = svm
        .simulate_transaction(native_increment_transaction)
        .expect("native increment simulation should succeed");
    let anchor_simulation = svm
        .simulate_transaction(anchor_increment_transaction)
        .expect("Anchor increment simulation should succeed");

    let native_units_consumed = native_simulation.meta.compute_units_consumed;
    let anchor_units_consumed = anchor_simulation.meta.compute_units_consumed;

    println!("native increment units_consumed: {native_units_consumed}");
    println!("Anchor increment units_consumed: {anchor_units_consumed}");

    assert!(native_units_consumed > 0);
    assert!(anchor_units_consumed > 0);
    assert_eq!(read_native_counter(&svm, &native_state.pubkey()), 0);
    assert_eq!(read_anchor_counter(&svm, &anchor_counter.pubkey()).value, 0);
}

#[test]
fn increment_on_a_nonexistent_account() {
    let mut svm = LiteSVM::new();
    svm.add_program_from_file(counter::ID, deployed_program_path("counter.so"))
        .expect("Anchor program should load");

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000)
        .expect("payer airdrop should succeed");

    let increment = Instruction {
        program_id: counter::ID,
        accounts: counter::accounts::Increment {
            counter: Pubkey::new_unique(),
        }
        .to_account_metas(None),
        data: counter::instruction::Increment.data(),
    };
    let transaction = Transaction::new_signed_with_payer(
        &[increment],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    let failure = svm
        .simulate_transaction(transaction)
        .expect_err("incrementing a nonexistent account should fail");

    assert_eq!(
        failure.err,
        TransactionError::InstructionError(0, InstructionError::Custom(3012))
    );
}
