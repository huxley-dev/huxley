use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TagRowModel {
    pub tag_id: Uuid,
    pub tag_type: String,
    pub name: String,
    pub bg_color: String,
    pub text_color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TagPublicModel {
    pub tag_id: Uuid,
    pub tag_type: String,
    pub name: String,
    pub bg_color: String,
    pub text_color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
