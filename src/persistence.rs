use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::{ NewItem, Item};
use uuid::Uuid;

pub struct PostgresRepository {
  pool: PgPool,
}

impl PostgresRepository {
  pub async fn connect(url: String) -> Self {
    Self {
      pool: PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap()
    }
  }

  pub async fn create_item(&self, new_item: NewItem) -> Result<Item, sqlx::Error> {
    sqlx::query_as(
      "
      INSERT INTO items (id, name, description, price, merchant_name)
      VALUES ($1, $2, $3, $4, $5)
      RETURNING id, name, description, price, merchant_name
      "
    )
    .bind(Uuid::new_v4())
    .bind(new_item.name)
    .bind(new_item.description)
    .bind(new_item.price)
    .bind(new_item.merchant_name)
    .fetch_one(&self.pool)
    .await
  }

  pub async fn list_items(&self) -> Result<Vec<Item>, sqlx::Error> {
    sqlx::query_as(
      "
      SELECT id, name, description, price, merchant_name
      FROM items
      LIMIT 50
      "
    )
    .fetch_all(&self.pool)
    .await
  }
}