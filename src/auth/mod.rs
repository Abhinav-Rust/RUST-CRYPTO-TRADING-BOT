use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

type HmacSha256 = Hmac<Sha256>;

/// Errors that can occur during the header generation process.
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Failed to generate system time")]
    TimeError(#[from] std::time::SystemTimeError),
    #[error("Failed to initialize HMAC")]
    HmacError(String),
    #[error("Failed to construct header value")]
    HeaderError(#[from] reqwest::header::InvalidHeaderValue),
}

/// API credentials required for authenticating with KuCoin.
pub struct Credentials {
    pub api_key: String,
    pub api_secret: String,
    pub api_passphrase: String,
}

/// Generates the standard `HeaderMap` required to authenticate requests to KuCoin's V2 API.
///
/// Implements HMAC-SHA256 double-signing authentication using the provided `Credentials`.
/// Expects the HTTP `method`, request `endpoint` (without the base domain), and an optional `body` (empty string for GET requests).
pub fn generate_auth_headers(
    method: &str,
    endpoint: &str,
    body: &str,
    credentials: &Credentials,
) -> Result<HeaderMap, AuthError> {
    let mut headers = HeaderMap::new();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis()
        .to_string();

    let str_to_sign = format!("{}{}{}{}", timestamp, method, endpoint, body);

    let mut mac = HmacSha256::new_from_slice(credentials.api_secret.as_bytes())
        .map_err(|e| AuthError::HmacError(e.to_string()))?;
    mac.update(str_to_sign.as_bytes());
    let signature = BASE64.encode(mac.finalize().into_bytes());

    let mut pass_mac = HmacSha256::new_from_slice(credentials.api_secret.as_bytes())
        .map_err(|e| AuthError::HmacError(e.to_string()))?;
    pass_mac.update(credentials.api_passphrase.as_bytes());
    let pass_signature = BASE64.encode(pass_mac.finalize().into_bytes());

    headers.insert("KC-API-KEY", HeaderValue::from_str(&credentials.api_key)?);
    headers.insert("KC-API-SIGN", HeaderValue::from_str(&signature)?);
    headers.insert("KC-API-TIMESTAMP", HeaderValue::from_str(&timestamp)?);
    headers.insert("KC-API-PASSPHRASE", HeaderValue::from_str(&pass_signature)?);
    headers.insert("KC-API-KEY-VERSION", HeaderValue::from_static("2"));

    Ok(headers)
}

#[cfg(test)]
mod tests;
