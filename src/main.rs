mod api;
mod trace;
mod utils;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use api::auth::{self, login, token_validator, verify, Keys};
use api::distributed_tracing::{child, parent};
use api::users::handlers::{create, delete, update, user_by_id, user_types, users};
use api::ws_handlers::echo_ws;

use tracing::info;
use tracing_actix_web::TracingLogger;

#[derive(derive_more::Constructor)]
struct AppState {
    child_port: u16,
    client: reqwest_middleware::ClientWithMiddleware,
    keys: Keys,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    //use tracing_subscriber;
    //tracing_subscriber::fmt().init();
    trace::init_tracing();

    info!(key = "value", "Hello");

    let keys = auth::generate_keys();

    info!("keys generated ok");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState::new(
                utils::child_port(),
                utils::client(),
                keys.clone(),
            )))
            .service(web::resource("/ws").route(web::get().to(echo_ws)))
            .service(
                web::scope("/v1")
                    .service(user_types)
                    .service(user_by_id)
                    .service(create)
                    .service(update)
                    .service(delete)
                    .service(users)
                    .service(parent)
                    .service(child)
                    .wrap(HttpAuthentication::bearer(token_validator)),
            )
            .service(login)
            .service(
                web::scope("")
                    .service(verify)
                    .wrap(HttpAuthentication::bearer(token_validator)),
            )
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .wrap(actix_web_opentelemetry::RequestTracing::new())
    })
    .bind(("127.0.0.1", utils::port()))?
    .run()
    .await
}
