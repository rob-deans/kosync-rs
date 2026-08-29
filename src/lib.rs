use toasty::stmt::Uuid;
pub mod app_error;

#[derive(Debug, toasty::Model)]
pub struct User {
    #[key]
    #[auto]
    id: Uuid,

    #[unique]
    pub username: String,
    pub passkey: String,
}

#[derive(Debug, toasty::Model)]
#[unique(document_id, username)]
pub struct BookProgress {
    #[key]
    pub document_id: String,
    pub username: String,

    pub percentage: f32,
    pub progress: String,
    pub device: String,
    #[auto]
    pub created_at: jiff::Timestamp,
    #[auto]
    pub updated_at: jiff::Timestamp,
}

pub async fn build_db() -> toasty::Result<toasty::Db> {
    let url = std::env::var("SQLITE_DB").unwrap_or_else(|_| "sqlite::memory:".to_string());

    toasty::Db::builder()
        .models(toasty::models!(crate::*))
        .max_pool_size(32)
        .pool_pre_ping(true) // check a pooled connection is alive before handing it out
        .connect(&url)
        .await
}
