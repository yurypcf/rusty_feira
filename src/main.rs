use std::{sync::Arc, env, net::SocketAddr};
use axum::{
  routing::{get, post},
  Router,
  response::IntoResponse,
  http::StatusCode, extract::State,
  Json,
};
use persistence::PostgresRepository;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

mod persistence;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Item {
  pub id: Uuid,
  pub name: String,
  pub description: String,
  pub price: f64,
  pub merchant_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewItem {
  pub name: String,
  pub description: String,
  pub price: f64,
  pub merchant_name: String,
}

type AppState = Arc<PostgresRepository>;

#[tokio::main]
async fn main() {

  let port = env::var("PORT")
    .ok()
    .and_then(|port| port.parse::<u16>().ok())
    .unwrap_or(9999);

  let database_url = env::var("DATABASE_URL")
    .unwrap_or(String::from("postgres://feiradorolo:123456@localhost:5432/feiradorolo"));

  let postgres_repo = persistence::PostgresRepository::connect(database_url).await;

  let app_state: AppState = Arc::new(postgres_repo);

  let app = Router::new()
    .route("/itens-a-venda", get(list_items))
    .route("/adicionar-item", post(create_item))
    .with_state(app_state);


  axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn list_items(
  State(items): State<AppState>
) -> impl IntoResponse {
  match items.list_items().await {
    Ok(items_result) => Ok(Json(items_result.clone())),
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
  }
}

async fn create_item(
  State(items): State<AppState>,
  Json(new_item): Json<NewItem>
) -> impl IntoResponse {
  match items.create_item(new_item).await {
    Ok(item) => Ok((StatusCode::CREATED, Json(item))),
    Err(err) => Err(StatusCode::UNPROCESSABLE_ENTITY),
  }
}