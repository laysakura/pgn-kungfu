use std::convert::TryInto;

use aes::{cipher::generic_array::GenericArray, Aes128, BlockDecrypt, NewBlockCipher};

use crate::host::enc_integer_avg::{DecodeError, EncInteger};

// 128-bit key
const MASTER_KEY: [u8; 16] = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

pub(in crate::enclave) trait EncIntegerDecoder {
    fn decode(self) -> Result<i32, DecodeError>;

    fn _decrypt(encrypted: Vec<u8>) -> Result<i32, DecodeError>;
}

impl EncIntegerDecoder for EncInteger {
    fn decode(self) -> Result<i32, DecodeError> {
        let decoded = base64::decode(self.base64)?;
        Self::_decrypt(decoded)
    }

    fn _decrypt(encrypted: Vec<u8>) -> Result<i32, DecodeError> {
        let key = GenericArray::from_slice(&MASTER_KEY);
        let mut enc_block = GenericArray::clone_from_slice(&encrypted);

        let cipher = Aes128::new(&key);

        let dec_block = {
            cipher.decrypt_block(&mut enc_block);
            enc_block
        };
        let decrypted = dec_block.to_vec();
        let decrypted: [u8; 4] = decrypted.try_into().map_err(|orig_vec: Vec<u8>| {
            DecodeError::new(format!(
                "base64-decoded data is {} bytes (while expected to be 4 bytes)",
                orig_vec.len()
            ))
        })?;

        Ok(i32::from_be_bytes(decrypted))
    }
}
