use solana_program::program_error::ProgramError;


pub fn double_u64(value: u64) -> Result<u64, ProgramError> {
    value.checked_mul(2).ok_or(ProgramError::ArithmeticOverflow)
}

pub fn parse_args(data: &[u8]) -> Result<(u8, u64), ProgramError> {
    let selector = data
        .first()
        .copied()
        .ok_or(ProgramError::InvalidInstructionData)?;
    let rest = data.get(1..9).ok_or(ProgramError::InvalidInstructionData)?;
    let bytes: [u8; 8] = rest
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let value = u64::from_le_bytes(bytes);
    Ok((selector, value))
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

    #[test]
    fn test_parse_arg() {
        let data = [1, 5, 0, 0, 0, 0, 0, 0, 0];
        let res = parse_args(&data).unwrap();
        assert_eq!(res, (1, 5));
    }

    #[test]
    fn test_parse_arg_negative() {
        let data = [1, 5, 0, 0, 0, 0, 0, 0];
        let err = parse_args(&data).unwrap_err();
        assert_eq!(err, ProgramError::InvalidInstructionData);
    }
}
