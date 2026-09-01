use axum::{
  routing::{get, post},
  Router, Json,
  extract::{State, Path},
  http::StatusCode,
  response::IntoResponse,
};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
  id: u64,
  name: String,
  email: String,
}

type UserStore = Arc<Mutex<HashMap<u64, User>>>;

#[tokio::main]
async fn main() {
  let store: UserStore = Arc::new(Mutex::new(HashMap::new()));
  
  let app = Router::new()
      .route("/", get(home))
      .route("/users", post(create_user).get(list_users))
      .route("/users/{id}", get(get_user).delete(delete_user))
      .with_state(store);
  
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  println!("Server running on http://localhost:3000");
  axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
  "REST API Running!"
}

async fn create_user(
  State(store): State<UserStore>,
  Json(user): Json<User>,
) -> impl IntoResponse {
  let mut users = store.lock().unwrap();
  users.insert(user.id, user.clone());
  (StatusCode::CREATED, Json(user))
}

async fn get_user(
  State(store): State<UserStore>,
  Path(id): Path<u64>,
) -> impl IntoResponse {
  let users = store.lock().unwrap();
  match users.get(&id) {
      Some(user) => (StatusCode::OK, Json(user.clone())),
      None => (StatusCode::NOT_FOUND, Json(User {
          id: 0,
          name: "".to_string(),
          email: "".to_string(),
      })),
  }
}

async fn list_users(State(store): State<UserStore>) -> impl IntoResponse {
  let users = store.lock().unwrap();
  let user_list: Vec<User> = users.values().cloned().collect();
  Json(user_list)
}

async fn delete_user(
  State(store): State<UserStore>,
  Path(id): Path<u64>,
) -> StatusCode {
  let mut users = store.lock().unwrap();
  if users.remove(&id).is_some() {
      StatusCode::NO_CONTENT
  } else {
      StatusCode::NOT_FOUND
  }
}