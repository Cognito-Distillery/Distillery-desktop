use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection, OptionalExtension};

use crate::models::Malt;

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub fn init_db(app_data_dir: &Path) -> Connection {
    std::fs::create_dir_all(app_data_dir).expect("failed to create app data dir");
    let db_path = app_data_dir.join("distillery.db");
    let conn = Connection::open(db_path).expect("failed to open database");
    conn.pragma_update(None, "journal_mode", "WAL").ok();
    create_schema(&conn);
    conn
}

fn create_schema(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS malts (
            id TEXT PRIMARY KEY,
            type TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'MALT_HOUSE',
            summary TEXT NOT NULL,
            context TEXT NOT NULL DEFAULT '',
            memo TEXT NOT NULL DEFAULT '',
            server_id TEXT,
            synced_at INTEGER,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS malts_fts USING fts5(
            summary, context, memo,
            content='malts',
            content_rowid='rowid',
            tokenize='trigram'
        );

        CREATE TRIGGER IF NOT EXISTS malts_ai AFTER INSERT ON malts BEGIN
            INSERT INTO malts_fts(rowid, summary, context, memo)
            VALUES (new.rowid, new.summary, new.context, new.memo);
        END;

        CREATE TRIGGER IF NOT EXISTS malts_ad AFTER DELETE ON malts BEGIN
            INSERT INTO malts_fts(malts_fts, rowid, summary, context, memo)
            VALUES ('delete', old.rowid, old.summary, old.context, old.memo);
        END;

        CREATE TRIGGER IF NOT EXISTS malts_au AFTER UPDATE ON malts BEGIN
            INSERT INTO malts_fts(malts_fts, rowid, summary, context, memo)
            VALUES ('delete', old.rowid, old.summary, old.context, old.memo);
            INSERT INTO malts_fts(rowid, summary, context, memo)
            VALUES (new.rowid, new.summary, new.context, new.memo);
        END;

        CREATE TABLE IF NOT EXISTS auth_tokens (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            email TEXT NOT NULL,
            access_token TEXT NOT NULL,
            refresh_token TEXT NOT NULL,
            updated_at INTEGER NOT NULL
        );
        ",
    )
    .expect("failed to create schema");
}

pub fn get_malts_by_status(
    conn: &Connection,
    status: &str,
    query: Option<&str>,
) -> Result<Vec<Malt>, String> {
    match query.filter(|q| !q.trim().is_empty()) {
        Some(q) => {
            let mut stmt = conn
                .prepare(
                    "SELECT m.id, m.type, m.status, m.summary, m.context, m.memo,
                            m.server_id, m.synced_at, m.created_at, m.updated_at
                     FROM malts m
                     JOIN malts_fts f ON m.rowid = f.rowid
                     WHERE m.status = ?1 AND malts_fts MATCH ?2
                     ORDER BY m.created_at DESC",
                )
                .map_err(|e| e.to_string())?;
            let rows = stmt
                .query_map(params![status, q], row_to_malt)
                .map_err(|e| e.to_string())?;
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())
        }
        None => {
            let mut stmt = conn
                .prepare(
                    "SELECT id, type, status, summary, context, memo,
                            server_id, synced_at, created_at, updated_at
                     FROM malts
                     WHERE status = ?1
                     ORDER BY created_at DESC",
                )
                .map_err(|e| e.to_string())?;
            let rows = stmt
                .query_map(params![status], row_to_malt)
                .map_err(|e| e.to_string())?;
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())
        }
    }
}

pub fn add_malt(
    conn: &Connection,
    malt_type: &str,
    summary: &str,
    context: &str,
    memo: &str,
) -> Result<Malt, String> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = now_ms();
    conn.execute(
        "INSERT INTO malts (id, type, status, summary, context, memo, created_at, updated_at) VALUES (?1, ?2, 'MALT_HOUSE', ?3, ?4, ?5, ?6, ?7)",
        params![id, malt_type, summary, context, memo, now, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(Malt {
        id,
        malt_type: malt_type.to_string(),
        status: "MALT_HOUSE".to_string(),
        summary: summary.to_string(),
        context: context.to_string(),
        memo: memo.to_string(),
        server_id: None,
        synced_at: None,
        created_at: now,
        updated_at: now,
    })
}

pub fn delete_malt(conn: &Connection, id: &str) -> Result<(), String> {
    conn.execute("DELETE FROM malts WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn update_malt(
    conn: &Connection,
    id: &str,
    malt_type: Option<&str>,
    summary: Option<&str>,
    context: Option<&str>,
    memo: Option<&str>,
) -> Result<Malt, String> {
    let now = now_ms();

    let mut sets = vec!["updated_at = ?1".to_string()];
    let mut param_idx = 2u32;

    if malt_type.is_some() {
        sets.push(format!("type = ?{}", param_idx));
        param_idx += 1;
    }
    if summary.is_some() {
        sets.push(format!("summary = ?{}", param_idx));
        param_idx += 1;
    }
    if context.is_some() {
        sets.push(format!("context = ?{}", param_idx));
        param_idx += 1;
    }
    if memo.is_some() {
        sets.push(format!("memo = ?{}", param_idx));
        param_idx += 1;
    }

    let sql = format!(
        "UPDATE malts SET {} WHERE id = ?{}",
        sets.join(", "),
        param_idx
    );

    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    param_values.push(Box::new(now));
    if let Some(v) = malt_type {
        param_values.push(Box::new(v.to_string()));
    }
    if let Some(v) = summary {
        param_values.push(Box::new(v.to_string()));
    }
    if let Some(v) = context {
        param_values.push(Box::new(v.to_string()));
    }
    if let Some(v) = memo {
        param_values.push(Box::new(v.to_string()));
    }
    param_values.push(Box::new(id.to_string()));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

    conn.execute(&sql, param_refs.as_slice())
        .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, type, status, summary, context, memo,
                    server_id, synced_at, created_at, updated_at
             FROM malts WHERE id = ?1",
        )
        .map_err(|e| e.to_string())?;
    stmt.query_row(params![id], row_to_malt)
        .map_err(|e| e.to_string())
}

pub fn set_malt_status(conn: &Connection, id: &str, status: &str) -> Result<(), String> {
    let now = now_ms();
    conn.execute(
        "UPDATE malts SET status = ?1, updated_at = ?2 WHERE id = ?3",
        params![status, now, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn search_malts(conn: &Connection, query: &str) -> Result<Vec<Malt>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT m.id, m.type, m.status, m.summary, m.context, m.memo,
                    m.server_id, m.synced_at, m.created_at, m.updated_at
             FROM malts m
             JOIN malts_fts f ON m.rowid = f.rowid
             WHERE malts_fts MATCH ?1
             ORDER BY m.created_at DESC",
        )
        .map_err(|e| e.to_string())?;
    let malts = stmt
        .query_map(params![query], row_to_malt)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(malts)
}

fn row_to_malt(row: &rusqlite::Row) -> rusqlite::Result<Malt> {
    Ok(Malt {
        id: row.get(0)?,
        malt_type: row.get(1)?,
        status: row.get(2)?,
        summary: row.get(3)?,
        context: row.get(4)?,
        memo: row.get(5)?,
        server_id: row.get(6)?,
        synced_at: row.get(7)?,
        created_at: row.get(8)?,
        updated_at: row.get(9)?,
    })
}

pub fn save_tokens(
    conn: &Connection,
    email: &str,
    access_token: &str,
    refresh_token: &str,
) -> Result<(), String> {
    let now = now_ms();
    conn.execute(
        "INSERT INTO auth_tokens (id, email, access_token, refresh_token, updated_at)
         VALUES (1, ?1, ?2, ?3, ?4)
         ON CONFLICT(id) DO UPDATE SET
            email = excluded.email,
            access_token = excluded.access_token,
            refresh_token = excluded.refresh_token,
            updated_at = excluded.updated_at",
        params![email, access_token, refresh_token, now],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_tokens(conn: &Connection) -> Result<Option<(String, String, String)>, String> {
    let mut stmt = conn
        .prepare("SELECT email, access_token, refresh_token FROM auth_tokens WHERE id = 1")
        .map_err(|e| e.to_string())?;
    let result = stmt
        .query_row([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .optional()
        .map_err(|e| e.to_string())?;
    Ok(result)
}

pub fn clear_tokens(conn: &Connection) -> Result<(), String> {
    conn.execute("DELETE FROM auth_tokens", [])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn mark_malts_queued(
    conn: &Connection,
    items: &[(String, String, i64)],
) -> Result<(), String> {
    let mut stmt = conn
        .prepare(
            "UPDATE malts SET server_id = ?1, synced_at = ?2, status = 'DISTILLED_READY', updated_at = ?3 WHERE id = ?4",
        )
        .map_err(|e| e.to_string())?;
    let now = now_ms();
    for (local_id, server_id, synced_at) in items {
        stmt.execute(params![server_id, synced_at, now, local_id])
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn mark_malt_drawn_back(conn: &Connection, local_id: &str) -> Result<(), String> {
    let now = now_ms();
    conn.execute(
        "UPDATE malts SET status = 'ON_STILL', server_id = NULL, synced_at = NULL, updated_at = ?1 WHERE id = ?2",
        params![now, local_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
