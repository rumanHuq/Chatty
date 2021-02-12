use crate::models::{Active, User};
use crate::utils::handle_psql_error;
use common::chat::{chat_server::Chat as ChatTrait, UserInput, UserSchema};
use sqlx::{self, Pool, Postgres};
use std::{ops::Deref, sync::Arc};
use tonic::{Request, Response, Status};

pub struct Chat {
  pub db: Arc<Pool<Postgres>>,
}

#[tonic::async_trait]
impl ChatTrait for Chat {
  async fn create_user(&self, request: Request<UserInput>) -> Result<Response<UserSchema>, Status> {
    let user = request.get_ref();
    let active = match &user.active {
      1 => Active::OffLine,
      2 => Active::Active,
      3 => Active::NotSeen,
      _ => Active::Unspecified,
    };

    let inserted_user: User =
      sqlx::query_as::<Postgres, User>("INSERT INTO users(name, active) VALUES($1,$2) RETURNING id, name, active")
        .bind(&user.name)
        .bind(active)
        .fetch_one(self.db.deref())
        .await
        .or_else(|e| return Err(handle_psql_error(e)))?;

    Ok(Response::new(UserSchema {
      id: inserted_user.id,
      name: inserted_user.name,
      active: user.active,
    }))
  }
}
