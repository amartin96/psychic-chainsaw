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

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_simple()
        .unwrap();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::Registry::default().with(filter).with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::trace_span!("server").in_scope(|| tracing::warn!("server started"));

    tonic::transport::Server::builder()
        .add_service(greeter_server::GreeterServer::new(GreeterService {}))
        .serve("[::1]:50051".parse().unwrap())
        .await
        .unwrap();
}
