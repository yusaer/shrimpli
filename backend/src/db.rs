use rand::Rng;
use sqlx::PgPool;

use crate::models::Url;

const SHORT_CODE_LENGTH: usize = 6;
const SHORT_CODE_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// Generate a random short code
fn generate_short_code() -> String {
    let mut rng = rand::thread_rng();
    (0..SHORT_CODE_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..SHORT_CODE_CHARS.len());
            SHORT_CODE_CHARS[idx] as char
        })
        .collect()
}

/// Create a new shortened URL
pub async fn create_url(pool: &PgPool, original_url: &str) -> Result<Url, sqlx::Error> {
    // Try to generate a unique short code (retry if collision)
    loop {
        let short_code = generate_short_code();

        let result = sqlx::query_as::<_, Url>(
            r#"
            INSERT INTO urls (short_code, original_url)
            VALUES ($1, $2)
            RETURNING id, short_code, original_url, clicks, created_at
            "#,
        )
        .bind(&short_code)
        .bind(original_url)
        .fetch_one(pool)
        .await;

        match result {
            Ok(url) => return Ok(url),
            Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
                // Collision, retry with a new code
                continue;
            }
            Err(e) => return Err(e),
        }
    }
}

/// Get URL by short code
pub async fn get_url_by_code(pool: &PgPool, short_code: &str) -> Result<Option<Url>, sqlx::Error> {
    sqlx::query_as::<_, Url>(
        r#"
        SELECT id, short_code, original_url, clicks, created_at
        FROM urls
        WHERE short_code = $1
        "#,
    )
    .bind(short_code)
    .fetch_optional(pool)
    .await
}

/// Increment click count and return the URL
pub async fn increment_clicks(pool: &PgPool, short_code: &str) -> Result<Option<Url>, sqlx::Error> {
    sqlx::query_as::<_, Url>(
        r#"
        UPDATE urls
        SET clicks = clicks + 1
        WHERE short_code = $1
        RETURNING id, short_code, original_url, clicks, created_at
        "#,
    )
    .bind(short_code)
    .fetch_optional(pool)
    .await
}
