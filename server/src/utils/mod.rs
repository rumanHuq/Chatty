pub mod env;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tonic::Status;

pub fn handle_psql_error(e: sqlx::error::Error) -> Status {
  // let a = e.try_into();
  let db_err = e.as_database_error();

  if let Some(error) = db_err {
    if let Some(code) = error.code().as_deref() {
      let status = match code {
        // https://github.com/lib/pq/blob/master/error.go all the error codes can be found here
        "23505" => Status::already_exists("User Already exists, please login"),
        _ => Status::unknown(format!("unknown error: {}", e)),
      };
      return status;
    } else {
      Status::unknown(format!("unknown error: {}", e))
    }
  } else {
    Status::unknown(format!("unknown error{}", e))
  }
}

pub async fn connect_to_db() -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
  let env_vars = env::EnvVars::get();
  println!("{}", env_vars.db);
  let db = PgPoolOptions::new()
    .max_connections(5)
    .connect(&env_vars.db[..])
    .await?;
  Ok(db)
}
