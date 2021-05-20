use aes::{cipher::generic_array::GenericArray, Aes128, BlockEncrypt, NewBlockCipher};

use crate::host::enc_integer_avg::EncInteger;

use super::MASTER_KEY;

pub(in crate::enclave) trait EncIntegerEncrypt {
    fn encrypt(integer: i32) -> Self;
}

impl EncIntegerEncrypt for EncInteger {
    fn encrypt(integer: i32) -> Self {
        let key = GenericArray::from_slice(&MASTER_KEY);
        let mut plain_block = GenericArray::clone_from_slice(&integer.to_be_bytes());

        let cipher = Aes128::new(&key);

        let enc_block = {
            cipher.encrypt_block(&mut plain_block);
            plain_block
        };
        Self::from(enc_block.to_vec())
    }
}
