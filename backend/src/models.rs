use serde::{Deserialize, Serialize};

/// Request body for URL shortening
#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

/// Response for URL shortening
#[derive(Debug, Serialize)]
pub struct ShortenResponse {
    pub short_code: String,
    pub short_url: String,
}

/// Response for URL stats
#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub short_code: String,
    pub original_url: String,
    pub clicks: i64,
}

/// Database model for URL
#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Url {
    pub id: i32,
    pub short_code: String,
    pub original_url: String,
    pub clicks: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
