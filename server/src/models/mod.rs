use sqlx::{self, types::chrono, FromRow, Type};
#[derive(FromRow, Debug)]
pub struct User {
  pub id: i64,
  pub name: String,
  pub active: Active,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Type, Debug)]
#[sqlx(type_name = "active", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Active {
  Unspecified,
  OffLine,
  Active,
  NotSeen,
}

impl Active {
  pub fn into_i32(&self) -> i32 {
    match self {
      Active::OffLine => 1,
      Active::Active => 2,
      Active::NotSeen => 3,
      Active::Unspecified => 0,
    }
  }
}

impl std::default::Default for Active {
  fn default() -> Self {
    Active::Unspecified
  }
}
