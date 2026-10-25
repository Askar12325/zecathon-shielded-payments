use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

pub fn encode_memo_base64url(memo: &str) -> String {
    URL_SAFE_NO_PAD.encode(memo.as_bytes())
}

pub fn decode_memo_base64url(encoded: &str) -> Result<String, String> {
    let bytes = URL_SAFE_NO_PAD
        .decode(encoded)
        .map_err(|e| format!("Base64url decode error: {e}"))?;
    String::from_utf8(bytes).map_err(|e| format!("UTF-8 decode error: {e}"))
}
