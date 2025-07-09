// backend/src/models/billing.rs
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct Invoice {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub appointment_id: Option<Uuid>,
    pub total_amount: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::billing_invoices)]
pub struct NewInvoice {
    pub patient_id: Uuid,
    pub appointment_id: Option<Uuid>,
    pub total_amount: f64,
    pub status: String,
}

#[derive(Queryable, Serialize)]
pub struct BillingItem {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub description: String,
    pub amount: f64,
    pub quantity: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::billing_items)]
pub struct NewBillingItem {
    pub invoice_id: Uuid,
    pub description: String,
    pub amount: f64,
    pub quantity: i32,
}

#[derive(Queryable, Serialize)]
pub struct Payment {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub amount: f64,
    pub payment_method: String,
    pub transaction_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::billing_payments)]
pub struct NewPayment {
    pub invoice_id: Uuid,
    pub amount: f64,
    pub payment_method: String,
    pub transaction_id: Option<String>,
}

#[derive(Queryable, Serialize)]
pub struct Discount {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub description: String,
    pub amount: f64,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::billing_discounts)]
pub struct NewDiscount {
    pub invoice_id: Uuid,
    pub description: String,
    pub amount: f64,
    pub approved_by: Option<Uuid>,
}