use opentelemetry::trace::{Span, Tracer};

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
        // tracing::info!("hello from server");

        let tracer = opentelemetry::global::tracer("tracer");
        let mut span = tracer.span_builder("span").start(&tracer);
        span.add_event("event", vec![]);
        span.end();

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(tonic::Response::new(reply))
    }
}

#[tokio::main]
async fn main() {
    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    // tracing::subscriber::set_global_default(subscriber).unwrap();

    let exporter = opentelemetry_stdout::SpanExporter::default();
    let provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_simple_exporter(exporter)
        // .with_config(opentelemetry_sdk::trace::Config::default())
        .build();
    opentelemetry::global::set_tracer_provider(provider);

    let addr = "[::1]:50051".parse().unwrap();
    let greeter = GreeterService {};

    tonic::transport::Server::builder()
        .add_service(greeter_server::GreeterServer::new(greeter))
        .serve(addr)
        .await
        .unwrap();
}
