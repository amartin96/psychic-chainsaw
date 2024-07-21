tonic::include_proto!("greeter");

#[tokio::main]
async fn main() {
    let mut client = greeter_client::GreeterClient::connect("http://localhost:50051")
        .await
        .unwrap();

    let request = HelloRequest { name: "world".into() };
    let result = client.say_hello(request).await.unwrap().into_inner();
    println!("{result:?}");
}
