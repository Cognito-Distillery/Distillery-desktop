use serde::{Deserialize, Serialize};

use crate::models::Malt;

const API_BASE_URL: &str = env!("API_BASE_URL");

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMalt {
    pub id: String,
    pub local_id: String,
    #[serde(default)]
    pub synced_at: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct DrawBackResponse {
    pub id: String,
    pub local_id: String,
}

#[derive(Debug, Deserialize)]
pub struct QueueResponse {
    pub malts: Vec<ServerMalt>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct QueueMaltPayload {
    local_id: String,
    #[serde(rename = "type")]
    malt_type: String,
    summary: String,
    context: String,
    memo: String,
    created_at: i64,
    updated_at: i64,
}

#[derive(Serialize)]
struct QueueRequest {
    malts: Vec<QueueMaltPayload>,
}

pub async fn queue_malt(
    access_token: &str,
    lang: &str,
    malt: &Malt,
) -> Result<ServerMalt, String> {
    let payload = QueueMaltPayload {
        local_id: malt.id.clone(),
        malt_type: malt.malt_type.clone(),
        summary: malt.summary.clone(),
        context: malt.context.clone(),
        memo: malt.memo.clone(),
        created_at: malt.created_at,
        updated_at: malt.updated_at,
    };
    let client = reqwest::Client::new();
    let url = format!("{}/malts", API_BASE_URL);
    let resp = client
        .post(&url)
        .bearer_auth(access_token)
        .header("Accept-Language", lang)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("queue failed ({}): {}", status, body));
    }
    resp.json::<ServerMalt>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn queue_malts(
    access_token: &str,
    lang: &str,
    malts: &[Malt],
) -> Result<QueueResponse, String> {
    let payload = QueueRequest {
        malts: malts
            .iter()
            .map(|m| QueueMaltPayload {
                local_id: m.id.clone(),
                malt_type: m.malt_type.clone(),
                summary: m.summary.clone(),
                context: m.context.clone(),
                memo: m.memo.clone(),
                created_at: m.created_at,
                updated_at: m.updated_at,
            })
            .collect(),
    };
    let client = reqwest::Client::new();
    let url = format!("{}/malts/batch", API_BASE_URL);
    let resp = client
        .post(&url)
        .bearer_auth(access_token)
        .header("Accept-Language", lang)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("queue failed ({}): {}", status, body));
    }
    resp.json::<QueueResponse>()
        .await
        .map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueuedMalt {
    pub id: String,
    pub local_id: String,
    #[serde(rename = "type")]
    pub malt_type: String,
    pub summary: String,
    pub context: String,
    pub memo: String,
    pub status: String,
    pub synced_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct QueuedMaltsResponse {
    pub malts: Vec<QueuedMalt>,
}

pub async fn get_queued_malts(access_token: &str, lang: &str) -> Result<QueuedMaltsResponse, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/malts?status=DISTILLED_READY", API_BASE_URL);
    let resp = client
        .get(&url)
        .bearer_auth(access_token)
        .header("Accept-Language", lang)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("get queued malts failed ({}): {}", status, body));
    }
    resp.json::<QueuedMaltsResponse>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn draw_back_malt(
    access_token: &str,
    lang: &str,
    server_id: &str,
) -> Result<DrawBackResponse, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/malts/{}", API_BASE_URL, server_id);
    let resp = client
        .delete(&url)
        .bearer_auth(access_token)
        .header("Accept-Language", lang)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("draw back failed ({}): {}", status, body));
    }
    resp.json::<DrawBackResponse>()
        .await
        .map_err(|e| e.to_string())
}
