mod db;
pub mod env;
use common::chat::{
  chat_server::{Chat as ChatTrait, ChatServer},
  UserInput, UserSchema,
};
use db::Db;
use serde::Serialize;
use sqlx::{self, FromRow, Pool, Postgres, Type};
use std::{ops::Deref, sync::Arc};
use tonic::{transport::Server, Request, Response, Status};
struct Chat {
  db: Arc<Pool<Postgres>>,
}

#[derive(FromRow, Debug, Serialize)]
struct User {
  id: i64,
  name: String,
  active: Active,
}
#[derive(Type, Debug, Serialize)]
#[sqlx(type_name = "active", rename_all = "SCREAMING_SNAKE_CASE")]
enum Active {
  Unspecified,
  OffLine,
  Active,
  NotSeen,
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
        .or_else(|e| Err(Status::unknown(format!("{}", e))))?;

    Ok(Response::new(UserSchema {
      id: inserted_user.id,
      name: inserted_user.name,
      active: user.active,
    }))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let db = Arc::new(Db::pool().await?);
  let chat_service = ChatServer::new(Chat { db: db.clone() });
  let addr = "[::0]:50051".parse()?;
  println!("GreeterServer listening on {}", addr);

  Server::builder().add_service(chat_service).serve(addr).await?;

  Ok(())
}
