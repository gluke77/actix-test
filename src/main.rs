mod api;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use api::handlers::{create, delete, echo_ws, update, user_by_id, user_types, users};
// use opentelemetry::global;
// use opentelemetry::trace::TracerProvider as _;
// use opentelemetry_otlp::WithExportConfig;
// use opentelemetry_sdk::runtime::Tokio;
use tracing::info;
use tracing_actix_web::TracingLogger;
// use tracing_subscriber::fmt;
// use tracing_subscriber::layer::SubscriberExt;
// use tracing_subscriber::util::SubscriberInitExt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let tracer_provider = opentelemetry_otlp::new_pipeline()
    //     .tracing()
    //     .with_exporter(
    //         opentelemetry_otlp::new_exporter()
    //             .http()
    //             .with_endpoint("127.0.0.1:4318"),
    //     )
    //     .install_batch(Tokio)
    //     .unwrap();

    // global::set_tracer_provider(tracer_provider.clone());

    // turn our OTLP pipeline into a tracing layer
    // let otel_layer =
    //     tracing_opentelemetry::layer().with_tracer(tracer_provider.tracer("otlp-tracer-level"));

    // let fmt_layer = fmt::layer().with_file(true).with_line_number(true);

    // tracing_subscriber::registry()
    //     // .with(otel_layer)
    //     .with(fmt_layer)
    //     // .with(ErrorTracingLayer::new())
    //     .init();

    tracing_subscriber::fmt().init();
    // tracing_subscriber::registry().with(fmt::layer()).init();

    info!(key = "value", "Hello");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .service(web::resource("/ws").route(web::get().to(echo_ws)))
            .service(user_types)
            .service(user_by_id)
            .service(create)
            .service(update)
            .service(delete)
            .service(users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
