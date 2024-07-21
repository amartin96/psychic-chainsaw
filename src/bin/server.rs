tonic::include_proto!("greeter");

pub struct GreeterService {}

#[tonic::async_trait]
impl greeter_server::Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> Result<tonic::Response<HelloReply>, tonic::Status> {
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(tonic::Response::new(reply))
    }
}

#[tokio::main]
async fn main() {

    
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = GreeterService {};

    tonic::transport::Server::builder()
        .add_service(greeter_server::GreeterServer::new(greeter))
        .serve(addr)
        .await
        .unwrap();
}
