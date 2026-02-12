use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Malt {
    pub id: String,
    #[serde(rename = "type")]
    pub malt_type: String,
    pub status: String,
    pub summary: String,
    pub context: String,
    pub memo: String,
    pub server_id: Option<String>,
    pub synced_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}
