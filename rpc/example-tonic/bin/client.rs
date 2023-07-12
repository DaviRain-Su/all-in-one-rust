use example_tonic::helloworld::greeter_client::GreeterClient;
use example_tonic::helloworld::HelloRequest;
use tonic::transport::Channel;

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
