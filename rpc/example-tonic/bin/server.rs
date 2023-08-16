use tonic::{transport::Server, Request, Response, Status};

use example_tonic::helloworld::greeter_server::{Greeter, GreeterServer};
use example_tonic::helloworld::{HelloReply, HelloRequest};

// Implementation of the Greeter service trait generated from the Protobuf definitions.
pub struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);
        let reply = example_tonic::helloworld::HelloReply {
            message: format!("Hello {}!", request.into_inner().name), // Create reply
        };
        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter {};

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
