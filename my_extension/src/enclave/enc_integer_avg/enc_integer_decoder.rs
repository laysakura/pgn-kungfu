use crate::host::enc_integer_avg::{DecodeError, EncInteger};

pub(in crate::enclave) trait EncIntegerDecoder {
    fn decode(self) -> Result<i32, DecodeError>;

    fn _decrypt(bin: Vec<u8>) -> Result<i32, DecodeError>;
}

impl EncIntegerDecoder for EncInteger {
    fn decode(self) -> Result<i32, DecodeError> {
        let decoded = base64::decode(self.base64)?;
        Self::_decrypt(decoded)
    }

    fn _decrypt(_bin: Vec<u8>) -> Result<i32, DecodeError> {
        todo!()
    }
}
