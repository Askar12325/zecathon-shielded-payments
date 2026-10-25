use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InvoiceStatus {
    Pending,
    Paid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub amount: f64,
    pub amount_zatoshis: u64,
    pub memo: Option<String>,
    pub recipient_address: String,
    pub zip321_uri: String,
    pub status: InvoiceStatus,
    pub txid: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInvoiceRequest {
    pub amount: f64,
    pub memo: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateInvoiceResponse {
    pub invoice_id: String,
    pub zip321_uri: String,
    pub amount: f64,
    pub memo: Option<String>,
    pub status: InvoiceStatus,
    pub recipient_address: String,
}

impl From<Invoice> for CreateInvoiceResponse {
    fn from(inv: Invoice) -> Self {
        Self {
            invoice_id: inv.id,
            zip321_uri: inv.zip321_uri,
            amount: inv.amount,
            memo: inv.memo,
            status: inv.status,
            recipient_address: inv.recipient_address,
        }
    }
}
