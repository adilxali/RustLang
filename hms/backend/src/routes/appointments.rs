// backend/src/routes/appointments.rs
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::models::appointment::{Appointment, NewAppointment};
use crate::schema::appointments::dsl::*;

pub async fn create_appointment(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    req: web::Json<NewAppointment>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection");

    // Check for scheduling conflicts
    let conflict = appointments
        .filter(doctor_id.eq(req.doctor_id))
        .filter(status.eq("scheduled"))
        .filter(start_time.lt(req.end_time))
        .filter(end_time.gt(req.start_time))
        .first::<Appointment>(&conn)
        .optional()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    if conflict.is_some() {
        return HttpResponse::Conflict().json("Scheduling conflict detected");
    }

    let new_appointment = NewAppointment {
        status: "scheduled".to_string(),
        ..req.into_inner()
    };

    diesel::insert_into(appointments)
        .values(&new_appointment)
        .get_result::<Appointment>(&conn)
        .map(|appointment| HttpResponse::Ok().json(appointment))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn cancel_appointment(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    appointment_id: web::Path<Uuid>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection");

    diesel::update(appointments.find(*appointment_id))
        .set((status.eq("cancelled"), updated_at.eq(Utc::now())))
        .get_result::<Appointment>(&conn)
        .map(|appointment| HttpResponse::Ok().json(appointment))
        .map_err(|_| HttpResponse::NotFound().finish())
}

pub async fn get_doctor_appointments(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    doctor_id: web::Path<Uuid>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection");

    appointments
        .filter(doctor_id.eq(*doctor_id))
        .filter(start_time.ge(Utc::now()))
        .order(start_time.asc())
        .load::<Appointment>(&conn)
        .map(|results| HttpResponse::Ok().json(results))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}