use opentelemetry::trace::TracerProvider as _;
use tracing_subscriber::layer::SubscriberExt as _;

tonic::include_proto!("greeter");

#[derive(Debug)]
pub struct GreeterService {}

#[tonic::async_trait]
impl greeter_server::Greeter for GreeterService {
    #[tracing::instrument]
    async fn say_hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> Result<tonic::Response<HelloReply>, tonic::Status> {
        tracing::info!("hello from server");

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(tonic::Response::new(reply))
    }
}

#[tokio::main]
async fn main() {
    let filter = tracing_subscriber::EnvFilter::try_new(format!(
        "{}=trace",
        env!("CARGO_BIN_NAME").replace("-", "_")
    ))
    .unwrap();

    let provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build();
    let tracer = provider.tracer("my-tracer-name");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::Registry::default().with(filter).with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let addr = "[::1]:50051".parse().unwrap();
    let greeter = GreeterService {};

    tonic::transport::Server::builder()
        .add_service(greeter_server::GreeterServer::new(greeter))
        .serve(addr)
        .await
        .unwrap();
}
