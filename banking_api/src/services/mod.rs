use crate::models::user::{RegisterInput, User, LoginInput};
use sqlx::PgPool;
use uuid::Uuid;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use argon2::password_hash::{SaltString};

use std::env;

pub async fn register_user(pool:&PgPool, input: RegisterInput) -> Result<User, String> {
    let hashed_password = hash_password(&input.password)?;

    let user = sqlx::query_as!(User,

        " INSERT INTO users (id, name, email, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name , email, password_hash, created_at",
        Uuid::new_v4().into(),
        input.email,
        hashed_password
    ).fetch_one(pool)
    .await
        .map_err(|e| e.to_string())?;
    Ok(user)
}

pub async fn login_user(pool: &PgPool, input: LoginInput) -> Result<User, String> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        input.email
    )
        .fetch_one(pool)
        .await
        .map_err(|_| "Invalid credentials".to_string())?;

    if verify_password(&input.password, &user.password_hash)? {
        Ok(user)
    } else {
        Err("Invalid credentials".to_string())
    }
}

fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?
        .to_string();

}

fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    argon2::verify_encoded(hash, password.as_bytes()).map_err(|e| e.to_string())
}