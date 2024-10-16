mod api;
mod trace;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use api::handlers::{
    child, create, delete, echo_ws, parent, update, user_by_id, user_types, users,
};

use tracing::info;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //use tracing_subscriber;
    //tracing_subscriber::fmt().init();
    trace::init_tracing();

    info!(key = "value", "Hello");

    let default_port = 8080;
    let port =
        std::env::var("PORT").map_or(default_port, |v| v.parse::<u16>().unwrap_or(default_port));

    HttpServer::new(|| {
        App::new()
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
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
