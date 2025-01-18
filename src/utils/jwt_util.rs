use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

const SECRET_KEY: &[u8] = b"secret";

pub fn generate(username: &str) -> String {
    let claims = Claims {
        sub: username.to_string(),
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
    };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(SECRET_KEY);

    encode(&header, &claims, &encoding_key).expect("Error generating JWT")
}

pub fn validate(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(SECRET_KEY);
    let mut validation = Validation::default();
    validation.validate_exp = true;
    validation.algorithms = vec![jsonwebtoken::Algorithm::HS256];

    decode::<Claims>(token, &decoding_key, &validation).map(|data| data.claims)
}
