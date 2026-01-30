use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use sqlx::PgPool;

use crate::db;
use crate::models::{ShortenRequest, ShortenResponse, StatsResponse};

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub base_url: String,
}

/// POST /api/shorten - Create a shortened URL
pub async fn shorten_url(
    State(state): State<AppState>,
    Json(req): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, AppError> {
    let url = db::create_url(&state.pool, &req.url).await?;

    Ok(Json(ShortenResponse {
        short_code: url.short_code.clone(),
        short_url: format!("{}/{}", state.base_url, url.short_code),
    }))
}

/// GET /{short_code} - Redirect to original URL
pub async fn redirect_url(
    State(state): State<AppState>,
    Path(short_code): Path<String>,
) -> Result<Response, AppError> {
    let url = db::increment_clicks(&state.pool, &short_code).await?;

    match url {
        Some(url) => Ok((StatusCode::FOUND, [(header::LOCATION, url.original_url)]).into_response()),
        None => Ok((StatusCode::NOT_FOUND, "URL not found").into_response()),
    }
}

/// GET /api/stats/{short_code} - Get URL statistics
pub async fn get_stats(
    State(state): State<AppState>,
    Path(short_code): Path<String>,
) -> Result<Response, AppError> {
    let url = db::get_url_by_code(&state.pool, &short_code).await?;

    match url {
        Some(url) => Ok(Json(StatsResponse {
            short_code: url.short_code,
            original_url: url.original_url,
            clicks: url.clicks,
        })
        .into_response()),
        None => Ok((StatusCode::NOT_FOUND, "URL not found").into_response()),
    }
}

/// Application error type
#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:?}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}
