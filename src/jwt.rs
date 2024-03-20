use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use hmac::{Hmac, Mac};
use json::JsonValue;
use sha2::Sha256;
use thiserror::Error;

type HmacSha256 = Hmac<Sha256>;

struct Jwt {
    header: JsonValue,
    payload: JsonValue,
    signature: String,
}

#[derive(Error, Debug)]
pub enum JWTError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("chrono error")]
    ChronoError,
}

fn create_header_and_payload(key: String, exp_seconds: i64) -> Result<(String, String), JWTError> {
    let header = json::object! {
        "alg": "HS256",
        "typ": "JWT",
        "sign_type": "SIGN",
    };

    let seconds = chrono::Duration::try_seconds(exp_seconds as i64).ok_or(JWTError::ChronoError)?;
    let now = chrono::Utc::now();
    let exp = now
        .checked_add_signed(seconds)
        .ok_or(JWTError::ChronoError)?;

    let payload = json::object! {
        "api_key": key,
        "exp": exp.timestamp() * 1000,
        "timestamp": now.timestamp() * 1000,
    };
    let encoded_header = URL_SAFE.encode(header.dump());
    let encoded_payload = URL_SAFE.encode(payload.dump());
    Ok((encoded_header, encoded_payload))
}

fn create_signature(key: &str, encoded_header: &str, encoded_payload: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(format!("{}.{}", encoded_header, encoded_payload).as_bytes());
    let result = mac.finalize();
    URL_SAFE.encode(result.into_bytes())
}

pub fn create_jwt(api_key: &str, exp_seconds: i64) -> Result<String, JWTError> {
    let parts = api_key.split(".").collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(JWTError::InvalidToken);
    }
    let id = parts[0];
    let key = parts[1];
    let (encoded_header, encoded_payload) = create_header_and_payload(id.to_string(), exp_seconds)?;
    let signature = create_signature(key, &encoded_header, &encoded_payload);

    Ok(format!(
        "{}.{}.{}",
        encoded_header, encoded_payload, signature
    ))
}
