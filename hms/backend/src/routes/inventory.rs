// backend/src/routes/inventory.rs
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::inventory::{NewInventoryItem, InventoryItem, NewInventoryTransaction, NewPrescription, NewPrescriptionItem, Prescription};
use crate::schema::{inventory_items, inventory_transactions, prescriptions, prescription_items};
use crate::models::billing::{NewInvoice, NewBillingItem};
use crate::schema::billing_invoices;

pub async fn add_inventory_item(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    item: web::Json<NewInventoryItem>,
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection");

    let result = conn.transaction(|conn| {
        let inserted_item = diesel::insert_into(inventory_items::table)
            .values(&item.into_inner())
            .get_result::<InventoryItem>(conn)?;

        diesel::insert_into(inventory_transactions::table)
            .values(NewInventoryTransaction {
                item_id: inserted_item.id,
                transaction_type: "add".to_string(),
                quantity: inserted_item.quantity,
                reason: Some("Initial stock".to_string()),
                created_by: *user_id,
            })
            .execute(conn)?;

        Ok(inserted_item)
    });

    result
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn process_prescription(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    prescription: web::Json<NewPrescription>,
    items: web::Json<Vec<NewPrescriptionItem>>,
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection");

    let result = conn.transaction(|conn| {
        let inserted_prescription = diesel::insert_into(prescriptions::table)
            .values(&NewPrescription {
                status: "pending".to_string(),
                ..prescription.into_inner()
            })
            .get_result::<Prescription>(conn)?;

        let mut total_amount = 0.0;
        for item in items.into_inner() {
            let inventory_item = inventory_items::table
                .find(item.item_id)
                .first::<InventoryItem>(conn)
                .map_err(|_| HttpResponse::NotFound().finish())?;

            if inventory_item.quantity < item.quantity {
                return Err(HttpResponse::BadRequest().json("Insufficient stock"));
            }

            diesel::insert_into(prescription_items::table)
                .values(&item)
                .execute(conn)?;

            diesel::update(inventory_items::table.find(item.item_id))
                .set((
                    inventory_items::quantity.eq(inventory_items::quantity - item.quantity),
                    inventory_items::updated_at.eq(chrono::Utc::now()),
                ))
                .execute(conn)?;

            diesel::insert_into(inventory_transactions::table)
                .values(NewInventoryTransaction {
                    item_id: item.item_id,
                    transaction_type: "remove".to_string(),
                    quantity: item.quantity,
                    reason: Some("Prescription fulfillment".to_string()),
                    created_by: *user_id,
                })
                .execute(conn)?;

            total_amount += inventory_item.unit_price * item.quantity as f64;
        }

        // Create billing invoice
        let new_invoice = NewInvoice {
            patient_id: inserted_prescription.patient_id,
            appointment_id: inserted_prescription.appointment_id,
            total_amount,
            status: "pending".to_string(),
        };

        diesel::insert_into(billing_invoices::table)
            .values(&new_invoice)
            .execute(conn)?;

        Ok(inserted_prescription)
    });

    result
        .map(|prescription| HttpResponse::Ok().json(prescription))
        .map_err(|err| err)
}

pub async fn get_low_stock_items(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection");

    inventory_items::table
        .filter(inventory_items::quantity.le(inventory_items::reorder_point))
        .load::<InventoryItem>(&conn)
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}