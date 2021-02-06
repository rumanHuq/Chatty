use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct Db;

impl Db {
  pub async fn pool() -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    let db = PgPoolOptions::new()
      .max_connections(5)
      .connect("postgres://localhost/chat_db")
      .await?;
    Ok(db)
  }
}
