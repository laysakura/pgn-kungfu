use pgx::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PostgresType)]
pub struct EncInteger {
    // TODO 固定長の文字列にしたい。4バイトのintegerをAES-128してBase64にすると...?
    // StealthDB だと `INTERNALLENGTH = 45` で指定している。
    base64: String,
}
