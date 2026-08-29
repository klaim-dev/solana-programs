mod instruction;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

use crate::instruction::{double_u64, parse_selector, parse_value, stateful_increment};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let selector = parse_selector(instruction_data)?;
    match selector {
        0 => stateful_increment(accounts, program_id),
        1 => {
            let value = parse_value(instruction_data)?;
            let doubled = double_u64(value)?;
            msg!("doubled value: {}", doubled);
            Ok(())
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

#[cfg(test)]
mod tests {

    use std::{assert_eq, str::FromStr, vec};

    use litesvm::LiteSVM;
    use solana_keypair::Keypair;
    use solana_program::instruction::{AccountMeta, Instruction};
    use solana_signer::Signer;
    use solana_transaction::Transaction;

    use super::*;

    #[test]
    fn test_unknown_selector() {
        let program_id = Pubkey::default();
        let accounts = [];
        let data = [99, 5, 0, 0, 0, 0, 0, 0, 0];

        let err = process_instruction(&program_id, &accounts, &data).unwrap_err();

        assert_eq!(err, ProgramError::InvalidInstructionData);
    }

    #[test]
    fn test_selector_0_requires_state_account() {
        let program_id = Pubkey::default();
        let accounts = [];
        let data = [0, 0, 0, 0, 0, 0, 0, 0, 0];

        let res = process_instruction(&program_id, &accounts, &data);
        assert_eq!(res, Err(ProgramError::NotEnoughAccountKeys));
    }

    #[test]
    fn test_selector_1() {
        let program_id = Pubkey::default();
        let accounts = [];
        let data = [1, 0, 0, 0, 0, 0, 0, 0, 0];

        let res = process_instruction(&program_id, &accounts, &data);
        assert_eq!(res, Ok(()));
    }

    #[test]
    fn test_overflow() {
        let program_id = Pubkey::default();
        let accounts = [];
        let data = [1, 255, 255, 255, 255, 255, 255, 255, 255];

        let err = process_instruction(&program_id, &accounts, &data).unwrap_err();

        assert_eq!(err, ProgramError::ArithmeticOverflow);
    }

    #[test]
    fn test_stateful() {
        let mut svm = LiteSVM::new();
        let program_id = Pubkey::from_str("AHPJLLL4YaEZyRaEUdEAvLWYC2p6XbrUW4j5gNFTtJUK").unwrap();
        svm.add_program_from_file(
            program_id,
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../target/deploy/hello_solana.so"
            ),
        )
        .unwrap();
        let state = Keypair::new();
        svm.airdrop(&state.pubkey(), 1_000_000_000).unwrap();
        let mut state_account = svm.get_account(&state.pubkey()).unwrap();
        state_account.data = vec![0u8; 16];
        state_account.owner =
            Pubkey::from_str(&"AHPJLLL4YaEZyRaEUdEAvLWYC2p6XbrUW4j5gNFTtJUK".to_string()).unwrap();
        state_account.executable = false;

        svm.set_account(state.pubkey(), state_account).unwrap();

        let data = vec![0];
        let accounts = vec![AccountMeta::new(state.pubkey(), false)];
        let instruction = Instruction {
            program_id,
            accounts,
            data,
        };

        let payer = Keypair::new();
        svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            svm.latest_blockhash(),
        );

        svm.send_transaction(tx).unwrap();
        let account = svm.get_account(&state.pubkey()).unwrap();
        let data = &account.data[..8];
        let bytes = data.try_into().unwrap();
        let value = u64::from_le_bytes(bytes);
        assert_eq!(value, 1);
    }
}
