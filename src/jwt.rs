use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    api_key: String,
    exp: i64,
    timestamp: i64,
}

#[derive(Error, Debug)]
pub enum JWTError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("chrono error")]
    ChronoError,
    #[error("json web token error")]
    JWTError(#[from] jsonwebtoken::errors::Error),
}

pub fn generate_token(api_key: &str, exp_seconds: i64) -> Result<String, JWTError> {
    let parts = api_key.split(".").collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(JWTError::InvalidToken);
    }
    let id = parts[0];
    let key = parts[1];
    let seconds = chrono::Duration::try_seconds(exp_seconds as i64).ok_or(JWTError::ChronoError)?;
    let now = chrono::Utc::now();
    let exp = now
        .checked_add_signed(seconds)
        .ok_or(JWTError::ChronoError)?;
    let claims = Claims {
        api_key: id.to_string(),
        exp: exp.timestamp(),
        timestamp: now.timestamp(),
    };
    let header = Header::new(Algorithm::HS256);
    encode(&header, &claims, &EncodingKey::from_secret(key.as_bytes())).map_err(JWTError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token() {
        let token =
            generate_token("04be3218a66194d58885178d8daf518e.oXNoeEp0C9Ehy93F", 3600).unwrap();
        println!("token: {}", token);
    }
}
