use opentelemetry::trace::TracerProvider;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions::resource::{SERVICE_NAME, SERVICE_VERSION};
use opentelemetry_semantic_conventions::SCHEMA_URL;
use tracing_opentelemetry;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{self, fmt};

fn init_tracer_provider(
) -> Result<opentelemetry_sdk::trace::TracerProvider, opentelemetry::trace::TraceError> {
    let resource = Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        ],
        SCHEMA_URL,
    );

    let trace_config = opentelemetry_sdk::trace::Config::default().with_resource(resource);

    let pipeline = opentelemetry_otlp::new_exporter()
        .tonic() // create GRPC layer
        .with_endpoint("http://localhost:4317");

    opentelemetry_otlp::new_pipeline()
        .tracing() // create OTLP tracing pipeline
        .with_exporter(pipeline)
        .with_trace_config(trace_config)
        .install_batch(Tokio) // configure a span exporter
}

pub fn init_tracing() {
    // is this only if we don't use tracing-opentelemetry?
    // opentelemetry::global::set_tracer_provider(tracer_provider.clone());

    // is this to propagate trace context across services?
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer_provider = init_tracer_provider().unwrap();

    let otel_layer =
        tracing_opentelemetry::layer().with_tracer(tracer_provider.tracer("otlp-tracer-level"));

    let fmt_layer = fmt::layer().with_file(true).with_line_number(true);
    let env_filter_layer = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(env_filter_layer)
        .with(otel_layer)
        .init();
}
