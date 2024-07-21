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
    let provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_simple_exporter(opentelemetry_otlp::new_exporter().tonic().build_span_exporter().unwrap())
        .build();
    opentelemetry::global::set_tracer_provider(provider);

    tonic::transport::Server::builder()
        .add_service(greeter_server::GreeterServer::new(GreeterService {}))
        .serve("[::1]:50051".parse().unwrap())
        .await
        .unwrap();
}
