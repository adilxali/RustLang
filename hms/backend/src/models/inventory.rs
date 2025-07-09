// backend/src/models/inventory.rs
use chrono::{DateTime, Utc, NaiveDate};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct InventoryItem {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub reorder_point: i32,
    pub expiry_date: Option<NaiveDate>,
    pub unit_price: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::inventory_items)]
pub struct NewInventoryItem {
    pub name: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub reorder_point: i32,
    pub expiry_date: Option<NaiveDate>,
    pub unit_price: f64,
}

#[derive(Queryable, Serialize)]
pub struct InventoryTransaction {
    pub id: Uuid,
    pub item_id: Uuid,
    pub transaction_type: String,
    pub quantity: i32,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::inventory_transactions)]
pub struct NewInventoryTransaction {
    pub item_id: Uuid,
    pub transaction_type: String,
    pub quantity: i32,
    pub reason: Option<String>,
    pub created_by: Uuid,
}

#[derive(Queryable, Serialize)]
pub struct Prescription {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub doctor_id: Uuid,
    pub appointment_id: Option<Uuid>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::prescriptions)]
pub struct NewPrescription {
    pub patient_id: Uuid,
    pub doctor_id: Uuid,
    pub appointment_id: Option<Uuid>,
    pub status: String,
}

#[derive(Queryable, Serialize)]
pub struct PrescriptionItem {
    pub id: Uuid,
    pub prescription_id: Uuid,
    pub item_id: Uuid,
    pub quantity: i32,
    pub dosage_instructions: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::prescription_items)]
pub struct NewPrescriptionItem {
    pub prescription_id: Uuid,
    pub item_id: Uuid,
    pub quantity: i32,
    pub dosage_instructions: Option<String>,
}