mod db;
use common::chat::{
  chat_server::{Chat as ChatTrait, ChatServer},
  User as UserInput, UserSchema,
};
use db::Db;
use sqlx::{self, FromRow, Pool, Postgres};
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};

struct Chat {
  db: Arc<Pool<Postgres>>,
}

#[derive(FromRow)]
struct User {
  name: String,
  active: i32,
}

#[tonic::async_trait]
impl ChatTrait for Chat {
  async fn create_user(&self, request: Request<UserInput>) -> Result<Response<UserSchema>, Status> {
    if let user = request.get_ref() {
      let db = self.db;
      let mut rows = sqlx::query("INSERT INTO users (username, active) VALUES (?,?)")
        .bind(&user.name)
        .bind(&user.active)
        .execute(db);
      unimplemented!();
    } else {
      Err(Status::invalid_argument("Provide some good shit"))
    }
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
