mod db;
use common::chat::{
  chat_server::{Chat as ChatTrait, ChatServer},
  User as UserInput,
};
use db::Db;
use sqlx::{self, FromRow, Pool, Postgres};
use std::{ops::Deref, sync::Arc};
use tonic::{transport::Server, Request, Response, Status};

struct Chat {
  db: Arc<Pool<Postgres>>,
}

#[derive(FromRow, Debug)]
struct User {
  id: i64,
  name: String,
  active: Active,
}
#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "active", rename_all = "SCREAMING_SNAKE_CASE")]
enum Active {
  Unspecified,
  OffLine,
  Active,
  NotSeen,
}

#[tonic::async_trait]
impl ChatTrait for Chat {
  async fn create_user(&self, request: Request<UserInput>) -> Result<Response<prost_types::Any>, Status> {
    let user = request.get_ref();
    let active = match &user.active {
      1 => Active::OffLine,
      2 => Active::Active,
      3 => Active::NotSeen,
      _ => Active::Unspecified,
    };

    let stream = sqlx::query_as::<_, User>("INSERT INTO users(name, active) VALUES($1,$2) RETURNING id, name, active")
      .bind(&user.name)
      .bind(active)
      .fetch_one(self.db.deref())
      .await
      .or_else(|e| Err(Status::unknown(format!("{}", e))))?;
    println!("=>{:?}", stream);
    unimplemented!();
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let db = Arc::new(Db::pool().await?);
  let chat_service = ChatServer::new(Chat { db: db.clone() });
  let addr = "[::1]:50051".parse()?;
  println!("GreeterServer listening on {}", addr);

  Server::builder().add_service(chat_service).serve(addr).await?;

  Ok(())
}
