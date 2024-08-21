use tonic::transport::Server;
use crate::server::chat_server::chat_server::ChatServer;
use crate::server::MyChat;

mod server;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyChat::default();

    Server::builder()
        .add_service(ChatServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
