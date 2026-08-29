use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn double_u64(value: u64) -> Result<u64, ProgramError> {
    value.checked_mul(2).ok_or(ProgramError::ArithmeticOverflow)
}

pub fn parse_selector(data: &[u8]) -> Result<u8, ProgramError> {
    let selector = data
        .first()
        .copied()
        .ok_or(ProgramError::InvalidInstructionData)?;
    Ok(selector)
}

pub fn parse_value(data: &[u8]) -> Result<u64, ProgramError> {
    let rest = data.get(1..9).ok_or(ProgramError::InvalidInstructionData)?;
    let bytes: [u8; 8] = rest
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let value = u64::from_le_bytes(bytes);
    Ok(value)
}

pub fn stateful_increment(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
) -> Result<(), ProgramError> {
    let mut iter = accounts.iter();
    let account = next_account_info(&mut iter)?;
    if account.owner != program_id {
        return Err(ProgramError::InvalidAccountOwner);
    };

    let mut borrow_data = account.try_borrow_mut_data()?;
    if borrow_data.len() < 8 {
        return Err(ProgramError::InvalidArgument);
    }
    let bytes = borrow_data[..8]
        .try_into()
        .map_err(|_| ProgramError::InvalidArgument)?;
    let data = u64::from_le_bytes(bytes);
    let new_value = data
        .checked_add(1)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let src = new_value.to_le_bytes();
    borrow_data[..8].copy_from_slice(&src);
    Ok(())
}

#[cfg(test)]
mod tests {

    use std::assert_eq;

    use super::*;

    #[test]
    fn test_double_u64() {
        let value = 5;
        let res = double_u64(value).unwrap();
        assert_eq!(res, 10);
    }

    #[test]
    fn test_double_u64_negative() {
        let value = u64::MAX;
        let err = double_u64(value).unwrap_err();
        assert_eq!(err, ProgramError::ArithmeticOverflow);
    }
}
