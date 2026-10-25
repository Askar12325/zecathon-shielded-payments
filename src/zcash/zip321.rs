use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::fmt;

/// Default testnet Unified Address for development and testing.
pub const DEFAULT_TESTNET_UA: &str =
    "utest1nvzd4wf8mxnext5mze9rzqc8feq2wpv27fdl396yy5lwaq6q6f3q6tnzw2h2ujtsgn9mnvgf6w9u7d4aq2dpxtnxespwlhyzavz9jujt";

#[derive(Debug, Clone, PartialEq)]
pub struct Zip321PaymentRequest {
    pub address: String,
    pub amount: f64,
    pub memo: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Zip321Error {
    InvalidAmount(f64),
    MemoTooLong(usize),
}

impl fmt::Display for Zip321Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Zip321Error::InvalidAmount(amt) => write!(f, "Amount must be positive, got {amt}"),
            Zip321Error::MemoTooLong(len) => write!(f, "Memo exceeds 512 bytes ({len} bytes)"),
        }
    }
}

impl std::error::Error for Zip321Error {}

impl Zip321PaymentRequest {
    pub fn new(address: impl Into<String>, amount: f64, memo: Option<String>) -> Result<Self, Zip321Error> {
        let address = address.into();
        if amount <= 0.0 {
            return Err(Zip321Error::InvalidAmount(amount));
        }
        if let Some(ref m) = memo {
            if m.as_bytes().len() > 512 {
                return Err(Zip321Error::MemoTooLong(m.as_bytes().len()));
            }
        }
        Ok(Self {
            address,
            amount,
            memo,
        })
    }

    /// Renders the payment request as a compliant ZIP-321 URI string:
    /// `zcash:<address>?amount=<amount>&memo=<base64url>`
    pub fn to_uri(&self) -> String {
        // Format amount with up to 8 decimal digits, trimming unnecessary trailing zeros
        let amount_str = format!("{:.8}", self.amount);
        let trimmed_amount = if amount_str.contains('.') {
            amount_str.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            amount_str
        };

        let mut uri = format!("zcash:{}?amount={}", self.address, trimmed_amount);

        if let Some(ref memo_text) = self.memo {
            if !memo_text.is_empty() {
                let encoded_memo = URL_SAFE_NO_PAD.encode(memo_text.as_bytes());
                uri.push_str(&format!("&memo={}", encoded_memo));
            }
        }

        uri
    }
}
