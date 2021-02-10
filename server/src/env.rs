use std::env;

pub struct EnvVars {
  pub db: String,
}

impl EnvVars {
  pub fn get() -> Self {
    let db = match env::var("DATABASE_URL") {
      Ok(db) => db,
      Err(_) => "postgres://localhost/chat_db".into(),
    };
    EnvVars { db }
  }
}
