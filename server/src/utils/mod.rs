mod env;
use env::EnvVars;
use sqlx::{postgres::PgPoolOptions, Error as sqlxError, Pool, Postgres};
use tonic::Status;

pub fn handle_psql_error(e: sqlxError) -> Status {
  let db_err = e.as_database_error();

  if let Some(error) = db_err {
    if let Some(code) = error.code().as_deref() {
      let status = match code {
        // https://github.com/lib/pq/blob/master/error.go all the error codes can be found here
        "23505" => Status::already_exists("User Already exists, please login"),
        num => Status::unknown(format!("db error with status: {:?}, error: {}", num, e)),
      };
      return status;
    } else {
      Status::unknown(format!("db error: {}", e))
    }
  } else {
    Status::unknown(format!("Schema Error: {}", e))
  }
}

pub async fn connect_to_db() -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
  let env_vars = EnvVars::get();
  let db = PgPoolOptions::new()
    .max_connections(5)
    .connect(&env_vars.db[..])
    .await?;
  Ok(db)
}
