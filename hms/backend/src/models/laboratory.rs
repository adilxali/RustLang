// backend/src/models/laboratory.rs
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct LabTest {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub reference_range: Option<String>,
    pub unit: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::lab_tests)]
pub struct NewLabTest {
    pub name: String,
    pub description: Option<String>,
    pub reference_range: Option<String>,
    pub unit: Option<String>,
}

#[derive(Queryable, Serialize)]
pub struct LabOrder {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub doctor_id: Uuid,
    pub appointment_id: Option<Uuid>,
    pub status: String,
    pub barcode: String,
    pub ordered_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::lab_orders)]
pub struct NewLabOrder {
    pub patient_id: Uuid,
    pub doctor_id: Uuid,
    pub appointment_id: Option<Uuid>,
    pub status: String,
    pub barcode: String,
}

#[derive(Queryable, Serialize)]
pub struct LabOrderTest {
    pub id: Uuid,
    pub order_id: Uuid,
    pub test_id: Uuid,
    pub status: String,
    pub result: Option<String>,
    pub notes: Option<String>,
    pub performed_by: Option<Uuid>,
    pub performed_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::lab_order_tests)]
pub struct NewLabOrderTest {
    pub order_id: Uuid,
    pub test_id: Uuid,
    pub status: String,
}

#[derive(Queryable, Serialize)]
pub struct LabQualityControl {
    pub id: Uuid,
    pub test_id: Uuid,
    pub metric_name: String,
    pub metric_value: f64,
    pub recorded_at: DateTime<Utc>,
    pub recorded_by: Uuid,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::lab_quality_control)]
pub struct NewLabQualityControl {
    pub test_id: Uuid,
    pub metric_name: String,
    pub metric_value: f64,
    pub recorded_by: Uuid,
}