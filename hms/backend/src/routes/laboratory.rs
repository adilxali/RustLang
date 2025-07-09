// backend/src/routes/laboratory.rs
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;
use rand::distributions::{Alphanumeric, DistString};

use crate::models::laboratory::{NewLabOrder, NewLabOrderTest, LabOrder, LabOrderTest, NewLabQualityControl};
use crate::schema::{lab_orders, lab_order_tests, lab_tests, lab_quality_control};
use crate::models::billing::{NewInvoice, NewBillingItem};
use crate::schema::billing_invoices;

pub async fn create_lab_order(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    order: web::Json<NewLabOrder>,
    tests: web::Json<Vec<NewLabOrderTest>>,
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection");

    let barcode = Alphanumeric.sample_string(&mut rand::thread_rng(), 12);
    let new_order = NewLabOrder {
        status: "ordered".to_string(),
        barcode,
        ..order.into_inner()
    };

    let result = conn.transaction(|conn| {
        let inserted_order = diesel::insert_into(lab_orders::table)
            .values(&new_order)
            .get_result::<LabOrder>(conn)?;

        let test_items = tests.into_inner();
        let mut total_amount = 0.0;
        for test in test_items {
            diesel::insert_into(lab_order_tests::table)
                .values(&test)
                .execute(conn)?;

            // Mock test price (replace with actual pricing logic)
            total_amount += 50.0; // Example price per test
        }

        // Create billing invoice
        let new_invoice = NewInvoice {
            patient_id: inserted_order.patient_id,
            appointment_id: inserted_order.appointment_id,
            total_amount,
            status: "pending".to_string(),
        };

        diesel::insert_into(billing_invoices::table)
            .values(&new_invoice)
            .execute(conn)?;

        Ok(inserted_order)
    });

    result
        .map(|order| HttpResponse::Ok().json(order))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn update_test_result(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    order_test_id: web::Path<Uuid>,
    update: web::Json<LabOrderTestUpdate>,
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection");

    let result = conn.transaction(|conn| {
        let updated_test = diesel::update(lab_order_tests::table.find(*order_test_id))
            .set((
                lab_order_tests::status.eq(&update.status),
                lab_order_tests::result.eq(&update.result),
                lab_order_tests::notes.eq(&update.notes),
                lab_order_tests::performed_by.eq(Some(*user_id)),
                lab_order_tests::performed_at.eq(Some(chrono::Utc::now())),
            ))
            .get_result::<LabOrderTest>(conn)?;

        // Update order status if all tests are completed
        let order_id = updated_test.order_id;
        let all_tests_completed = lab_order_tests::table
            .filter(lab_order_tests::order_id.eq(order_id))
            .filter(lab_order_tests::status.ne("completed"))
            .count()
            .get_result::<i64>(conn)? == 0;

        if all_tests_completed {
            diesel::update(lab_orders::table.find(order_id))
                .set((
                    lab_orders::status.eq("completed"),
                    lab_orders::updated_at.eq(chrono::Utc::now()),
                ))
                .execute(conn)?;
        }

        Ok(updated_test)
    });

    result
        .map(|test| HttpResponse::Ok().json(test))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn record_quality_control(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    qc: web::Json<NewLabQualityControl>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection");

    diesel::insert_into(lab_quality_control::table)
        .values(&qc.into_inner())
        .get_result::<LabQualityControl>(&conn)
        .map(|qc| HttpResponse::Ok().json(qc))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

#[derive(Deserialize)]
pub struct LabOrderTestUpdate {
    pub status: String,
    pub result: Option<String>,
    pub notes: Option<String>,
}