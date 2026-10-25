use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use zecpay::api::routes::{create_router, AppState};
use zecpay::config::Config;
use zecpay::scanner::detector::start_detector;
use zecpay::storage::InvoiceStore;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "zecpay=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    let store = InvoiceStore::new();

    // Spawn the background payment detector task
    start_detector(store.clone(), config.clone());

    let app_state = AppState {
        store,
        config: config.clone(),
    };

    let app = create_router(app_state);

    let addr: SocketAddr = format!("{}:{}", config.server_host, config.server_port)
        .parse()
        .expect("Valid socket address");

    tracing::info!("🚀 ZecPay server starting on http://{}", addr);
    tracing::info!("🔑 Merchant Testnet Address: {}", config.merchant_address);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind to {}: {}", addr, e));

    axum::serve(listener, app).await.expect("Server error");
}
