use serde::Serialize;
use sqlx::{FromRow, Type};

#[derive(FromRow, Debug, Serialize)]
pub struct User {
  pub id: i64,
  pub name: String,
  pub active: Active,
}
#[derive(Type, Debug, Serialize)]
#[sqlx(type_name = "active", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Active {
  Unspecified,
  OffLine,
  Active,
  NotSeen,
}
