use std::env;

pub struct EnvVars {
  pub db: String,
}

impl EnvVars {
  pub fn get() -> Self {
    let get_from_env_variables = |key: &str, fallback: &str| {
      return match env::var(key) {
        Ok(db) => db,
        Err(_) => fallback.into(),
      };
    };
    let db = get_from_env_variables("DATABASE_URL", "postgres://localhost/chat_db".into());
    EnvVars { db }
  }
}
