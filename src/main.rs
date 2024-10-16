mod api;

use actix_web::{App, HttpServer};
use api::handlers::{create, delete, update, user_by_id, user_types, users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
