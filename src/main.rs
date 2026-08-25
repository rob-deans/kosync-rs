use axum::{
    http::{HeaderMap, StatusCode},
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};

// TODO:
// Hook up sqlite
// Migrations/schema
// Read headers for auth

// user_key = "user:%s:key",
// doc_key = "user:%s:document:%s",

// User
#[derive(Serialize, Debug)]
struct User {
    username: String,
}

#[derive(Deserialize, Debug)]
struct RegisterUser {
    username: String,
    // TODO: Probably have to alias this
    password: String,
}

struct RequestUser {}

// Progress

#[derive(Serialize, Default, Debug)]
struct Progress {
    percentage: f32,
    progress: f32,
    device: String,
    device_id: String,
    timestamp: String, // TODO: Proper timestamp
}

#[derive(Deserialize, Debug)]
struct RequestProgress {
    username: String,
    userkey: String,
    document: String,
}

#[derive(Deserialize, Debug)]
struct UpdateProgress {
    username: String,
    userkey: String,
    document: String,
    percentage: f32,
    progress: f32,
    device: String,
}

// Health
#[derive(Serialize)]
struct HealthCheck {
    status: String,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app = Router::new()
        .route("/health", get(healthcheck))
        .route("/users/create", post(register))
        .route("/users/auth", post(auth))
        .route("/syncs/progress", put(sync_progress).get(get_progress));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    _ = axum::serve(listener, app).await;
}

// async fn users(Json(payload): Json<RequestUser>) -> (StatusCode, Json<User>) {}

// Aka create user
// Allow/disallow creating user from env var
//
async fn register(Json(payload): Json<RegisterUser>) -> (StatusCode, Json<User>) {
    // Reject on dupe
    // no auth!
    // Maybe 201?
    (
        StatusCode::OK,
        Json(User {
            username: "rob".into(),
        }),
    )
}

// x-auth-user
// x-auth-key
// might be internal only
// empty password:
// 401 , code 2001 on Unauthorised
// Bad password:
// same as above
// Good:
// 200, OK
async fn auth() {}

// Should auth first
// Non-existent
// 200, empty response
// Found:
// 200, with a timestamp
// Should always get the latest document
async fn get_progress(
    headers: HeaderMap,
    Json(payload): Json<RequestProgress>,
) -> (StatusCode, Json<Progress>) {
    println!("{headers:?}");
    // get key and match on result x2 for auth headers
    // middleware? technically nice for certain endpoints
    (StatusCode::OK, Json(Progress::default()))
}

// Should auth first
// Update
// seems like response is same as input with extra timestamp
async fn sync_progress(Json(payload): Json<UpdateProgress>) {
    // x-auth-user
    // x-auth-key
    println!("{payload:?}");
}

async fn healthcheck() -> (StatusCode, Json<HealthCheck>) {
    (
        StatusCode::OK,
        Json(HealthCheck {
            status: "ok".into(),
        }),
    )
}
