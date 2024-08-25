use chat_client::chat_client::ChatClient;
use chat_client::HelloRequest;

pub mod chat_client {
    tonic::include_proto!("chat");
}

pub async fn ask_hello() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChatClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}