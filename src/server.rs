pub mod chat {
  tonic::include_proto!("chat");
}
use chat::{
  chat_server::{Chat, ChatServer},
  ChatResults, Message, Register, User, Users,
};
mod db;
use db::Db;
use std::{pin::Pin, sync::Arc};
use tokio_stream::Stream;
use tonic::{transport::Server, Request, Response, Status};
struct ChatService {
  db: Arc<Db>,
}

#[tonic::async_trait]
impl Chat for ChatService {
  async fn change_status(&self, _request: Request<User>) -> Result<Response<()>, Status> {
    unimplemented!();
  }
  async fn get_active_people(&self, _request: Request<()>) -> Result<Response<Users>, Status> {
    unimplemented!();
  }
  async fn ping(&self, _request: Request<()>) -> Result<Response<()>, Status> {
    unimplemented!();
  }
  type RegisterAndChatStream = Pin<Box<dyn Stream<Item = Result<ChatResults, Status>> + Send + Sync + 'static>>;
  async fn register_and_chat(
    &self,
    _request: Request<Register>,
  ) -> Result<Response<Self::RegisterAndChatStream>, Status> {
    unimplemented!();
  }
  async fn send_message(&self, _request: Request<Message>) -> Result<Response<()>, Status> {
    unimplemented!();
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:10000".parse().unwrap();

  println!("RouteGuideServer listening on: {}", addr);
  let db = Db::new().await?;

  let chat = ChatService { db: Arc::new(db) };

  let svc = ChatServer::new(chat);

  Server::builder().add_service(svc).serve(addr).await?;

  Ok(())
}
