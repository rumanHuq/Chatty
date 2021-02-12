mod models;
mod services;
mod utils;

use common::chat::chat_server::ChatServer;
use services::chat::Chat;
use std::{error::Error, sync::Arc};
use tonic::transport::Server;
use utils::connect_to_db;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let addr = "[::0]:50051".parse()?;
  println!("💬💬 Chatty 💬💬 started on {}", addr);
  let db = Arc::new(connect_to_db().await?);
  // ----------------------------------------------------------------
  let chat_service = ChatServer::new(Chat { db: db.clone() });

  Server::builder().add_service(chat_service).serve(addr).await?;

  Ok(())
}
