//! Common routines for packing/unpacking

use {
    crate::string::ArrayString64,
    arrayref::{array_refs, mut_array_refs},
    serde::{
        de::{Error, Visitor},
        Deserialize, Deserializer, Serializer,
    },
    solana_program::{program_error::ProgramError, pubkey::Pubkey},
    std::{fmt, str::FromStr},
};

/// Checks if the slice has at least min_len size
pub fn check_data_len(data: &[u8], min_len: usize) -> Result<(), ProgramError> {
    if data.len() < min_len {
        Err(ProgramError::AccountDataTooSmall)
    } else {
        Ok(())
    }
}

pub fn pack_array_string64(input: &ArrayString64, output: &mut [u8; 64]) {
    for (dst, src) in output.iter_mut().zip(input.as_bytes()) {
        *dst = *src
    }
}

pub fn unpack_array_string64(input: &[u8; 64]) -> Result<ArrayString64, ProgramError> {
    if let Some(i) = input.iter().position(|x| *x == 0) {
        ArrayString64::try_from_utf8(&input[0..i]).or(Err(ProgramError::InvalidAccountData))
    } else {
        ArrayString64::try_from_utf8(input).or(Err(ProgramError::InvalidAccountData))
    }
}

/// Custom ArrayString64 deserializer to use with Serde
pub fn as64_deserialize<'de, D>(deserializer: D) -> Result<ArrayString64, D::Error>
where
    D: Deserializer<'de>,
{
    struct ArrayStringVisitor;

    impl<'de> Visitor<'de> for ArrayStringVisitor {
        type Value = ArrayString64;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            ArrayString64::try_from_str(v).map_err(E::custom)
        }
    }

    deserializer.deserialize_any(ArrayStringVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as64_serialization() {
        let as1 = ArrayString64::from_utf8("test").unwrap();
        let mut output: [u8; 64] = [0; 64];
        pack_array_string64(&as1, &mut output);
        let as2 = unpack_array_string64(&output).unwrap();
        assert_eq!(as1, as2);
    }

    #[test]
    fn test_as64_serialization_utf8() {
        let as1 = ArrayString64::from_utf8("тест").unwrap();
        let mut output: [u8; 64] = [0; 64];
        pack_array_string64(&as1, &mut output);
        let as2 = unpack_array_string64(&output).unwrap();
        assert_eq!(as1, as2);
    }
}
