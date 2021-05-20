use std::convert::TryInto;

use aes::{cipher::generic_array::GenericArray, Aes128, BlockDecrypt, NewBlockCipher};

use crate::host::enc_integer_avg::{DecodeError, EncInteger};

use super::MASTER_KEY;

pub(in crate::enclave) trait EncIntegerDecrypt {
    fn decrypt(self) -> Result<i32, DecodeError>;
}

impl EncIntegerDecrypt for EncInteger {
    fn decrypt(self) -> Result<i32, DecodeError> {
        let encrypted = self.as_slice();

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
