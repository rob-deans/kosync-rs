use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing_subscriber::EnvFilter;

use kosync_rs::app_error::AppError;
use kosync_rs::{BookProgress, User};

static MIGRATIONS: toasty::migration::MigrationSet = toasty::embed_migrations!();

// User
#[derive(Serialize, Debug, Default)]
struct UserDto {
    username: String,
}

#[derive(Deserialize, Debug)]
struct RegisterUserDto {
    username: String,
    password: String,
}

// Progress
#[derive(Serialize, Default, Debug)]
struct ProgressDto {
    percentage: f32,
    progress: String,
    device: String,
    timestamp: i64,
    document: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct UpdateProgressDto {
    document: String,
    progress: String,
    percentage: f32,
    device: String,
}

// Health
#[derive(Serialize)]
struct HealthCheck {
    state: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,toasty=debug".into()),
        )
        .init();
    let db = toasty::Db::builder()
        .models(toasty::models!(crate::*))
        .connect("sqlite:./kosync.db")
        .await
        .unwrap();

    let _ = migrate(&db).await;

    let app = Router::new()
        .route("/health", get(healthcheck))
        .route("/users/create", post(register))
        .route("/users/auth", get(auth_v2))
        .route("/syncs/progress", put(sync_progress))
        .route("/syncs/progress/{document}", get(get_progress))
        .with_state(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    _ = axum::serve(listener, app).await;
}

async fn migrate(db: &toasty::Db) -> toasty::Result<()> {
    let report = MIGRATIONS.apply(db).await?;
    println!("applied {} migrations", report.applied());
    Ok(())
}

// TODO: Allow/disallow creating user from env var
async fn register(
    State(mut db): State<toasty::Db>,
    Json(payload): Json<RegisterUserDto>,
) -> Result<(StatusCode, Json<UserDto>), AppError> {
    let result = toasty::create!(User {
        username: payload.username,
        passkey: payload.password,
    })
    .exec(&mut db)
    .await;
    match result {
        Ok(user) => Ok((
            StatusCode::CREATED, // The client expects 201
            Json(UserDto {
                username: user.username,
            }),
        )),
        Err(_) => Err(AppError::UserExists),
    }
}

async fn auth_v2(
    headers: HeaderMap,
    State(db): State<toasty::Db>,
) -> Result<Json<Value>, AppError> {
    match auth(headers, db).await {
        Ok(_) => Ok(Json(json!({"authorized": "OK"}))),
        Err(_) => Err(AppError::Auth),
    }
}

async fn auth(headers: HeaderMap, mut db: toasty::Db) -> Result<UserDto, AppError> {
    if let Some(auth_user) = headers.get("x-auth-user") && let Some(auth_key) = headers.get("x-auth-key") {
        match User::get_by_username(&mut db, auth_user.to_str().unwrap()).await {
            Ok(user) => {
                if auth_key.to_str().unwrap() != user.passkey {
                    return Err(AppError::Auth)
                }
                Ok(UserDto{username: user.username})
            }
            Err(_) => {
                Err(AppError::Auth)
            }
        }
    } else {
        Err(AppError::Auth)
    }
}

// Non-existent
// 200, empty response
async fn get_progress(
    headers: HeaderMap,
    State(mut db): State<toasty::Db>,
    Path(document): Path<String>,
) -> Result<Json<ProgressDto>, AppError> {
    let user = auth(headers, db.clone()).await?;

    match BookProgress::get_by_document_id_and_username(&mut db, document, user.username).await {
        Ok(p) => Ok(Json(ProgressDto {
            progress: p.progress,
            percentage: p.percentage,
            device: p.device,
            timestamp: p.updated_at.as_second(),
            document: p.document_id,
        })),
        Err(e) => {
            println!("{e:?}");
            if e.is_record_not_found() {
                return Err(AppError::BookNotFound);
            }
            Err(AppError::Internal)
        }
    }
}

async fn sync_progress(
    headers: HeaderMap,
    State(mut db): State<toasty::Db>,
    Json(payload): Json<UpdateProgressDto>,
) -> Result<Json<Value>, AppError> {
    let user = auth(headers, db.clone()).await?;

    match BookProgress::upsert_by_document_id_and_username(payload.document, &user.username)
        .percentage(payload.percentage)
        .progress(payload.progress)
        .device(payload.device)
        .exec(&mut db)
        .await
    {
        Ok(p) => Ok(Json(
            json!({"document":p.document_id, "timestamp": p.updated_at}),
        )),
        Err(_) => Err(AppError::Internal),
    }
}

async fn healthcheck() -> Result<Json<HealthCheck>, AppError> {
    Ok(Json(HealthCheck { state: "OK".into() }))
}
