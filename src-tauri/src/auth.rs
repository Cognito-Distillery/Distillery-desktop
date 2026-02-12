use serde::{Deserialize, Serialize};

const API_BASE_URL: &str = env!("API_BASE_URL");

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize)]
struct SendOtpRequest<'a> {
    email: &'a str,
}

#[derive(Serialize)]
struct VerifyOtpRequest<'a> {
    email: &'a str,
    otp: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RefreshRequest<'a> {
    refresh_token: &'a str,
}

pub async fn send_otp(email: &str, lang: &str) -> Result<(), String> {
    let client = reqwest::Client::new();
    let url = format!("{}/auth/send-otp", API_BASE_URL);
    let resp = client
        .post(&url)
        .header("Accept-Language", lang)
        .json(&SendOtpRequest { email })
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("send_otp failed ({}): {}", status, body));
    }
    Ok(())
}

pub async fn verify_otp(email: &str, otp: &str, lang: &str) -> Result<TokenResponse, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/auth/verify-otp", API_BASE_URL);
    let resp = client
        .post(&url)
        .header("Accept-Language", lang)
        .json(&VerifyOtpRequest { email, otp })
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("verify_otp failed ({}): {}", status, body));
    }
    resp.json::<TokenResponse>()
        .await
        .map_err(|e| e.to_string())
}

#[derive(Debug)]
pub enum RefreshError {
    Unauthorized,
    #[allow(dead_code)]
    Other(String),
}

pub async fn refresh_token(refresh: &str, lang: &str) -> Result<TokenResponse, RefreshError> {
    let client = reqwest::Client::new();
    let url = format!("{}/auth/refresh", API_BASE_URL);
    let resp = client
        .post(&url)
        .header("Accept-Language", lang)
        .json(&RefreshRequest {
            refresh_token: refresh,
        })
        .send()
        .await
        .map_err(|e| RefreshError::Other(e.to_string()))?;
    if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
        return Err(RefreshError::Unauthorized);
    }
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(RefreshError::Other(format!(
            "refresh failed ({}): {}",
            status, body
        )));
    }
    resp.json::<TokenResponse>()
        .await
        .map_err(|e| RefreshError::Other(e.to_string()))
}
