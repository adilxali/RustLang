// backend/src/routes/billing.rs
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::billing::{NewInvoice, NewBillingItem, NewPayment, NewDiscount, Invoice};
use crate::schema::{billing_invoices, billing_items, billing_payments, billing_discounts};

pub async fn create_invoice(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    req: web::Json<NewInvoice>,
    items: web::Json<Vec<NewBillingItem>>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection");

    let total = items.iter().fold(0.0, |acc, item| acc + item.amount * item.quantity as f64);

    let new_invoice = NewInvoice {
        total_amount: total,
        status: "pending".to_string(),
        ..req.into_inner()
    };

    let result = conn.transaction(|conn| {
        let invoice = diesel::insert_into(billing_invoices::table)
            .values(&new_invoice)
            .get_result::<Invoice>(conn)?;

        for item in items.into_inner() {
            diesel::insert_into(billing_items::table)
                .values(&item)
                .execute(conn)?;
        }
        Ok(invoice)
    });

    result
        .map(|invoice| HttpResponse::Ok().json(invoice))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn process_payment(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    payment: web::Json<NewPayment>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection");

    let invoice = billing_invoices::table
        .find(payment.invoice_id)
        .first::<Invoice>(&conn)
        .map_err(|_| HttpResponse::NotFound().finish())?;

    let paid_amount = billing_payments::table
        .filter(billing_payments::invoice_id.eq(payment.invoice_id))
        .select(diesel::dsl::sum(billing_payments::amount))
        .first::<Option<f64>>(&conn)
        .map_err(|_| HttpResponse::InternalServerError().finish())?
        .unwrap_or(0.0);

    let new_payment = payment.into_inner();

    let new_status = if paid_amount + new_payment.amount >= invoice.total_amount {
        "paid".to_string()
    } else if paid_amount + new_payment.amount > 0.0 {
        "partially_paid".to_string()
    } else {
        "pending".to_string()
    };

    let result = conn.transaction(|conn| {
        diesel::insert_into(billing_payments::table)
            .values(&new_payment)
            .execute(conn)?;

        diesel::update(billing_invoices::table.find(invoice.id))
            .set((
                billing_invoices::status.eq(new_status),
                billing_invoices::updated_at.eq(chrono::Utc::now()),
            ))
            .get_result::<Invoice>(conn)
    });

    result
        .map(|invoice| HttpResponse::Ok().json(invoice))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn apply_discount(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    discount: web::Json<NewDiscount>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection");

    let invoice = billing_invoices::table
        .find(discount.invoice_id)
        .first::<Invoice>(&conn)
        .map_err(|_| HttpResponse::NotFound().finish())?;

    let total_discount = billing_discounts::table
        .filter(billing_discounts::invoice_id.eq(discount.invoice_id))
        .select(diesel::dsl::sum(billing_discounts::amount))
        .first::<Option<f64>>(&conn)
        .map_err(|_| HttpResponse::InternalServerError().finish())?
        .unwrap_or(0.0);

    let new_total = invoice.total_amount - total_discount - discount.amount;

    let result = conn.transaction(|conn| {
        diesel::insert_into(billing_discounts::table)
            .values(&discount)
            .execute(conn)?;

        diesel::update(billing_invoices::table.find(invoice.id))
            .set((
                billing_invoices::total_amount.eq(new_total),
                billing_invoices::updated_at.eq(chrono::Utc::now()),
            ))
            .get_result::<Invoice>(conn)
    });

    result
        .map(|invoice| HttpResponse::Ok().json(invoice))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn get_patient_invoices(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    patient_id: web::Path<Uuid>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection");

    billing_invoices::table
        .filter(billing_invoices::patient_id.eq(*patient_id))
        .load::<Invoice>(&conn)
        .map(|invoices| HttpResponse::Ok().json(invoices))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}