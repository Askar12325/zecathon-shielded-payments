use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::Invoice;

#[derive(Clone, Default)]
pub struct InvoiceStore {
    invoices: Arc<RwLock<HashMap<String, Invoice>>>,
}

impl InvoiceStore {
    pub fn new() -> Self {
        Self {
            invoices: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, invoice: Invoice) {
        let mut lock = self.invoices.write().await;
        lock.insert(invoice.id.clone(), invoice);
    }

    pub async fn get(&self, id: &str) -> Option<Invoice> {
        let lock = self.invoices.read().await;
        lock.get(id).cloned()
    }

    pub async fn list(&self) -> Vec<Invoice> {
        let lock = self.invoices.read().await;
        let mut list: Vec<Invoice> = lock.values().cloned().collect();
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        list
    }

    pub async fn mark_as_paid(&self, id: &str, txid: Option<String>) -> bool {
        let mut lock = self.invoices.write().await;
        if let Some(inv) = lock.get_mut(id) {
            inv.status = crate::models::InvoiceStatus::Paid;
            inv.txid = txid;
            true
        } else {
            false
        }
    }
}
