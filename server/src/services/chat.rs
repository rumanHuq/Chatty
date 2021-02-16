use crate::models::User;
use crate::utils::{
  handle_psql_error,
  overrites::{ChronoToProstTime, ProtoActiveOverrite},
};
use common::chat::{chat_server::Chat as ChatTrait, UserInput, UserSchema};
use sqlx::{self, Pool, Postgres};
use std::{ops::Deref, pin::Pin, sync::Arc};
use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status};

pub struct Chat {
  pub db: Arc<Pool<Postgres>>,
}

#[tonic::async_trait]
impl ChatTrait for Chat {
  async fn create_user(&self, request: Request<UserInput>) -> Result<Response<UserSchema>, Status> {
    let user = request.get_ref();
    let active = user.active.into_enum();
    let inserted_user: User = sqlx::query_as::<Postgres, User>(
      "INSERT INTO users(name, active) VALUES($1,$2) RETURNING id, name, active, created_at",
    )
    .bind(&user.name)
    .bind(active)
    .fetch_one(self.db.clone().deref())
    .await
    .or_else(|e| return Err(handle_psql_error(e)))?;

    let created_at = inserted_user.created_at.into_prost_timestamp().normalize();
    println!("{:?}, {:?}", created_at, inserted_user.created_at);
    let response = UserSchema {
      id: inserted_user.id,
      name: inserted_user.name,
      active: user.active,
      created_at: Some(inserted_user.created_at.into_prost_timestamp()),
    };

    Ok(Response::new(response))
  }

  type GetUsersStream = Pin<Box<dyn Stream<Item = Result<UserSchema, Status>> + Send + Sync + 'static>>;
  async fn get_users(&self, _request: Request<()>) -> Result<Response<Self::GetUsersStream>, Status> {
    let db = self.db.deref();
    let result = sqlx::query!(r#"SELECT COUNT(*) as "count!" FROM users"#)
      .fetch_one(db)
      .await
      .or_else(|e| Err(handle_psql_error(e)))?;

    let (tx, rx) = mpsc::channel(result.count as usize);
    let mut stream = sqlx::query_as::<Postgres, User>("SELECT id,name,active,created_at FROM users").fetch(db);

    while let Some(user) = stream.try_next().await.or_else(|e| Err(handle_psql_error(e)))? {
      // map the row into a user-defined domain type
      let response = UserSchema {
        id: user.id,
        name: user.name,
        active: user.active.into_i32(),
        created_at: Some(user.created_at.into_prost_timestamp()),
      };
      tx.send(Ok(response.clone()))
        .await
        .or_else(|e| Err(Status::unknown(format!("{:?}", e))))?;
    }

    Ok(Response::new(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx))))
  }
}
