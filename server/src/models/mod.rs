use sqlx::{types::chrono, FromRow, Type};

#[derive(FromRow, Debug)]
pub struct User {
  pub id: i64,
  pub name: String,
  pub active: Active,
  pub created: chrono::DateTime<chrono::Utc>,
}
#[derive(Type, Debug)]
#[sqlx(type_name = "active", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Active {
  Unspecified,
  OffLine,
  Active,
  NotSeen,
}

impl std::default::Default for Active {
  fn default() -> Self {
    Active::Unspecified
  }
}
