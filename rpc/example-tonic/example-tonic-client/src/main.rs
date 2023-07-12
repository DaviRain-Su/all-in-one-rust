use helloworld::greeter_client::GreeterClient;
use helloworld::HelloRequest;
use tonic::transport::Channel;

pub mod helloworld {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the gRPC server running on localhost:50051
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;

    // Create a new GreeterClient
    let mut client = GreeterClient::new(channel);

    // Create a new HelloRequest
    let request = tonic::Request::new(HelloRequest {
        name: "World".into(),
    });

    // Make the SayHello RPC call using the client
    let response = client.say_hello(request).await?.into_inner();

    // Print out the response
    println!("RESPONSE={:?}", response);

    Ok(())
}
