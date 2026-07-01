//! User controller — full CRUD example.
//!
//! Register routes in `main.rs`:
//! ```rust
//! .route("/api/users",    get(list_users).post(create_user))
//! .route("/api/users/:id", get(get_user).put(update_user).delete(delete_user))
//! ```

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

/// User data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

/// Payload for creating / updating a user.
#[derive(Debug, Deserialize)]
pub struct UserPayload {
    pub name: String,
    pub email: String,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// GET /api/users — list all users.
pub async fn list_users() -> impl IntoResponse {
    // TODO: replace with real DB query
    let users: Vec<User> = vec![
        User {
            id: "00000000-0000-0000-0000-000000000001".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            created_at: Utc::now().to_rfc3339(),
        },
    ];
    (StatusCode::OK, Json(users))
}

/// POST /api/users — create a new user.
pub async fn create_user(Json(payload): Json<UserPayload>) -> impl IntoResponse {
    let user = User {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        email: payload.email,
        created_at: Utc::now().to_rfc3339(),
    };
    (StatusCode::CREATED, Json(user))
}

/// GET /api/users/:id — fetch a single user.
pub async fn get_user(Path(id): Path<String>) -> impl IntoResponse {
    // TODO: replace with real DB query
    let user = User {
        id,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        created_at: Utc::now().to_rfc3339(),
    };
    (StatusCode::OK, Json(user))
}

/// PUT /api/users/:id — update a user.
pub async fn update_user(
    Path(id): Path<String>,
    Json(payload): Json<UserPayload>,
) -> impl IntoResponse {
    let user = User {
        id,
        name: payload.name,
        email: payload.email,
        created_at: Utc::now().to_rfc3339(),
    };
    (StatusCode::OK, Json(user))
}

/// DELETE /api/users/:id — delete a user.
pub async fn delete_user(Path(id): Path<String>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({ "deleted": id })),
    )
}
