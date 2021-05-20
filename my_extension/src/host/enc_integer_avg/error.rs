use std::{error::Error, fmt::Display};

/// Enc* 型のデコード（含む複合）に失敗した際のエラー
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct DecodeError(String);

impl Error for DecodeError {}
impl Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl DecodeError {
    pub(crate) fn new(message: String) -> Self {
        Self(message)
    }
}
