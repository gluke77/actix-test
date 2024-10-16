mod api;
mod trace;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use api::handlers::{create, delete, echo_ws, update, user_by_id, user_types, users};

use tracing::info;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //use tracing_subscriber;
    //tracing_subscriber::fmt().init();
    trace::init_tracing();

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
