mod api;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use api::handlers::{create, delete, update, user_by_id, user_types, users};
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(user_types)
            .service(user_by_id)
            .service(create)
            .service(update)
            .service(delete)
            .service(users)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
