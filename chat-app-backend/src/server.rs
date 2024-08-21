use tonic::{transport::Server, Request, Response, Status};

use chat_server::chat_server::{Chat, ChatServer};
use chat_server::{HelloResponse, HelloRequest};

pub mod chat_server {
    tonic::include_proto!("chat"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyChat{
}

#[tonic::async_trait]
impl Chat for MyChat{
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloResponse {
            message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}