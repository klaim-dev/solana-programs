mod instruction;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

use crate::instruction::{double_u64, parse_args};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (selector, value) = parse_args(instruction_data)?;
    match selector {
        0 => {
            msg!("value: {}", value);
            Ok(())
        }
        1 => {
            let doubled = double_u64(value)?;
            msg!("doubled value: {}", doubled);
            Ok(())
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

#[cfg(test)]
mod tests {

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
    fn test_selector_0() {
        let program_id = Pubkey::default();
        let accounts = [];
        let data = [0, 0, 0, 0, 0, 0, 0, 0, 0];

        let res = process_instruction(&program_id, &accounts, &data);
        assert_eq!(res, Ok(()));
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
}
