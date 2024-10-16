mod api;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use api::handlers::{create, delete, echo_ws, update, user_by_id, user_types, users};
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{self, fmt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer().with_file(true).with_line_number(true))
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
    // tracing_subscriber::fmt().init();

    info!(key = "value", "Hello");

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/ws").route(web::get().to(echo_ws)))
            .service(user_types)
            .service(user_by_id)
            .service(create)
            .service(update)
            .service(delete)
            .service(users)
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
