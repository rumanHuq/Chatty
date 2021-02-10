use super::env::EnvVars;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct Db;

impl Db {
  pub async fn pool() -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    let env_vars = EnvVars::get();
    println!("{}", env_vars.db);
    let db = PgPoolOptions::new()
      .max_connections(5)
      .connect(&env_vars.db[..])
      .await?;
    Ok(db)
  }
}
