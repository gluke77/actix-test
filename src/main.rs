mod api;
mod trace;
mod utils;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use api::auth::token_validator;
use api::distributed_tracing::{child, parent};
use api::users::handlers::{create, delete, update, user_by_id, user_types, users};
use api::ws_handlers::echo_ws;

use tracing::info;
use tracing_actix_web::TracingLogger;

#[derive(derive_more::Constructor)]
struct AppState {
    child_port: u16,
    client: reqwest_middleware::ClientWithMiddleware,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //use tracing_subscriber;
    //tracing_subscriber::fmt().init();
    trace::init_tracing();

    info!(key = "value", "Hello");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::new(
                utils::child_port(),
                utils::client(),
            )))
            .service(web::resource("/ws").route(web::get().to(echo_ws)))
            .service(user_types)
            .service(user_by_id)
            .service(create)
            .service(update)
            .service(delete)
            .service(users)
            .service(parent)
            .service(child)
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .wrap(actix_web_opentelemetry::RequestTracing::new())
            .wrap(HttpAuthentication::bearer(token_validator))
    })
    .bind(("127.0.0.1", utils::port()))?
    .run()
    .await
}
