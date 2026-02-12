use std::sync::Mutex;

use rusqlite::Connection;
use tauri::{AppHandle, Manager, State};

use crate::api;
use crate::auth;
use crate::db;
use crate::models::Malt;

pub struct DbState(pub Mutex<Connection>);

fn validate_lang(lang: &str) -> Result<(), String> {
    match lang {
        "ko" | "en" => Ok(()),
        _ => Err(format!("unsupported language: {}", lang)),
    }
}

#[tauri::command]
pub fn get_malts_by_status(
    state: State<DbState>,
    status: String,
    query: Option<String>,
) -> Result<Vec<Malt>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_malts_by_status(&conn, &status, query.as_deref())
}

#[tauri::command]
pub fn add_malt(
    state: State<DbState>,
    malt_type: String,
    summary: String,
    context: String,
    memo: String,
) -> Result<Malt, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::add_malt(&conn, &malt_type, &summary, &context, &memo)
}

#[tauri::command]
pub fn delete_malt(state: State<DbState>, id: String) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::delete_malt(&conn, &id)
}

#[tauri::command]
pub fn update_malt(
    state: State<DbState>,
    id: String,
    malt_type: Option<String>,
    summary: Option<String>,
    context: Option<String>,
    memo: Option<String>,
) -> Result<Malt, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::update_malt(
        &conn,
        &id,
        malt_type.as_deref(),
        summary.as_deref(),
        context.as_deref(),
        memo.as_deref(),
    )
}

#[tauri::command]
pub fn set_malt_status(
    state: State<DbState>,
    id: String,
    status: String,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::set_malt_status(&conn, &id, &status)
}

#[tauri::command]
pub fn search_malts(state: State<DbState>, query: String) -> Result<Vec<Malt>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::search_malts(&conn, &query)
}

#[tauri::command]
pub async fn auth_send_otp(email: String, lang: String) -> Result<(), String> {
    validate_lang(&lang)?;
    auth::send_otp(&email, &lang).await
}

#[tauri::command]
pub async fn auth_verify_otp(
    state: State<'_, DbState>,
    email: String,
    otp: String,
    lang: String,
) -> Result<(), String> {
    validate_lang(&lang)?;
    let tokens = auth::verify_otp(&email, &otp, &lang).await?;
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::save_tokens(&conn, &email, &tokens.access_token, &tokens.refresh_token)
}

#[tauri::command]
pub async fn auth_check(state: State<'_, DbState>, lang: String) -> Result<Option<String>, String> {
    validate_lang(&lang)?;
    let stored = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        db::get_tokens(&conn)?
    };
    match stored {
        None => Ok(None),
        Some((email, _access, refresh)) => {
            match auth::refresh_token(&refresh, &lang).await {
                Ok(new_tokens) => {
                    let conn = state.0.lock().map_err(|e| e.to_string())?;
                    db::save_tokens(
                        &conn,
                        &email,
                        &new_tokens.access_token,
                        &new_tokens.refresh_token,
                    )?;
                    Ok(Some(email))
                }
                Err(auth::RefreshError::Unauthorized) => {
                    let conn = state.0.lock().map_err(|e| e.to_string())?;
                    db::clear_tokens(&conn)?;
                    Ok(None)
                }
                Err(_) => {
                    Ok(Some(email))
                }
            }
        }
    }
}

#[tauri::command]
pub async fn auth_logout(state: State<'_, DbState>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::clear_tokens(&conn)
}

#[tauri::command]
pub async fn get_queued_malts(
    state: State<'_, DbState>,
    lang: String,
) -> Result<Vec<Malt>, String> {
    validate_lang(&lang)?;
    let access_token = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        let tokens = db::get_tokens(&conn)?
            .ok_or_else(|| "로그인이 필요합니다".to_string())?;
        tokens.1
    };

    let resp = api::get_queued_malts(&access_token, &lang).await?;

    Ok(resp
        .malts
        .into_iter()
        .map(|m| Malt {
            id: m.local_id,
            malt_type: m.malt_type,
            status: m.status,
            summary: m.summary,
            context: m.context,
            memo: m.memo,
            server_id: Some(m.id),
            synced_at: Some(m.synced_at),
            created_at: m.created_at,
            updated_at: m.updated_at,
        })
        .collect())
}

#[tauri::command]
pub async fn malt_draw_back(
    state: State<'_, DbState>,
    id: String,
    server_id: String,
    lang: String,
) -> Result<(), String> {
    validate_lang(&lang)?;
    let access_token = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        let tokens = db::get_tokens(&conn)?
            .ok_or_else(|| "로그인이 필요합니다".to_string())?;
        tokens.1
    };

    api::draw_back_malt(&access_token, &lang, &server_id).await?;

    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::mark_malt_drawn_back(&conn, &id)?;

    Ok(())
}

#[tauri::command]
pub async fn malt_queue(
    state: State<'_, DbState>,
    id: String,
    lang: String,
) -> Result<(), String> {
    validate_lang(&lang)?;
    let (access_token, malt) = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        let tokens = db::get_tokens(&conn)?
            .ok_or_else(|| "로그인이 필요합니다".to_string())?;
        let all = db::get_malts_by_status(&conn, "ON_STILL", None)?;
        let malt = all.into_iter().find(|m| m.id == id)
            .ok_or_else(|| "해당 몰트를 찾을 수 없습니다".to_string())?;
        (tokens.1, malt)
    };

    let server_malt = api::queue_malt(&access_token, &lang, &malt).await?;

    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::mark_malts_queued(&conn, &[(server_malt.local_id, server_malt.id, server_malt.synced_at)])?;

    Ok(())
}

#[tauri::command]
pub async fn malts_queue_batch(
    state: State<'_, DbState>,
    lang: String,
) -> Result<u32, String> {
    validate_lang(&lang)?;
    let (access_token, malts) = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        let tokens = db::get_tokens(&conn)?
            .ok_or_else(|| "로그인이 필요합니다".to_string())?;
        let malts = db::get_malts_by_status(&conn, "ON_STILL", None)?;
        (tokens.1, malts)
    };

    if malts.is_empty() {
        return Ok(0);
    }

    let resp = api::queue_malts(&access_token, &lang, &malts).await?;

    let items: Vec<(String, String, i64)> = resp
        .malts
        .iter()
        .map(|m| (m.local_id.clone(), m.id.clone(), m.synced_at))
        .collect();

    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::mark_malts_queued(&conn, &items)?;

    Ok(items.len() as u32)
}

#[tauri::command]
pub fn get_web_url(state: State<DbState>) -> Result<String, String> {
    let base = env!("WEB_URL");
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let tokens = db::get_tokens(&conn)?
        .ok_or_else(|| "로그인이 필요합니다".to_string())?;
    let (_email, access, refresh) = tokens;
    Ok(format!("{}/login/callback?a={}&r={}", base, access, refresh))
}

#[tauri::command]
pub fn hide_floating_memo(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("floating-memo") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}
