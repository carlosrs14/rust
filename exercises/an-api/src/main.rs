use axum::{
    Json, Router, extract::{Path, State}, http::StatusCode, routing::{get, post},
};

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Clone)]
struct AppState {
    users: Arc<Mutex<HashMap<u64, User>>>,
}

async  fn get_users(State(state): State<AppState>,) -> Json<Vec<User>> {
    let users = state.users.lock().unwrap();
    Json(users.values().cloned().collect())
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.lock().unwrap();

    users
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Json<User> {
    let mut users = state.users.lock().unwrap();

    let id = users.len() as u64 + 1;

    let user = User {
        id,
        name: input.name,
        email: input.email,
    };

    users.insert(id, user.clone());

    Json(user)
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let mut users = state.users.lock().unwrap();

    let user = users
        .get_mut(&id)
        .ok_or(StatusCode::NOT_FOUND)?;

    user.name = input.name;
    user.email = input.email;

    Ok(Json(user.clone()))
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> StatusCode {
    let mut users = state.users.lock().unwrap();

    if users.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

#[tokio::main]
async fn main() {
    let state: AppState = AppState {
        users: Arc::new(Mutex::new(HashMap::new()))
    };

    let app = Router::new()
        .route("/api/v1/users", post(create_user).get(get_users))
        .route("/api/v1/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    
    axum::serve(listener, app)
        .await
        .unwrap();
}
