use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub merchant_address: String,
    pub lightwalletd_url: String,
    pub demo_auto_settle: bool,
    pub auto_settle_delay_secs: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            server_port: env::var("SERVER_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            merchant_address: env::var("MERCHANT_ADDRESS")
                .unwrap_or_else(|_| crate::zcash::zip321::DEFAULT_TESTNET_UA.into()),
            lightwalletd_url: env::var("LIGHTWALLETD_URL")
                .unwrap_or_else(|_| "https://testnet.zec.rocks:443".into()),
            demo_auto_settle: env::var("DEMO_AUTO_SETTLE")
                .map(|v| v.to_lowercase() == "true" || v == "1")
                .unwrap_or(true),
            auto_settle_delay_secs: env::var("AUTO_SETTLE_DELAY_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(15),
        }
    }
}
