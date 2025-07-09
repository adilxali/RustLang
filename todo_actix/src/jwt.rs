use jsonwebtoken::{decode, encode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,  // Subject (user ID or other identifier)
    exp: usize,   // Expiration time
}

pub fn create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = 10000000000; // Example expiration time
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(get_secret_key().as_bytes());

    encode(&header, &claims, &encoding_key)
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(get_secret_key().as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(token, &decoding_key, &validation).map(|data| data.claims)
}

fn get_secret_key() -> String {
    env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set")
}
