// backend/src/routes/auth.rs
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
    user: User,
}

pub async fn login(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    req: web::Json<LoginRequest>,
) -> HttpResponse {
    use crate::schema::users::dsl::*;
    let conn = pool.get().expect("Couldn't get db connection");

    let user = users
        .filter(email.eq(&req.email))
        .first::<User>(&conn)
        .map_err(|_| HttpResponse::Unauthorized().finish())?;

    if verify_password(&req.password, &user.password_hash) {
        let claims = Claims {
            sub: user.id.to_string(),
            role: user.role,
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp(),
        };
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
            .map_err(|_| HttpResponse::InternalServerError().finish())?;

        HttpResponse::Ok().json(LoginResponse { token, user })
    } else {
        HttpResponse::Unauthorized().finish()
    }
}