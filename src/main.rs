mod api;
mod trace;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use api::handlers::{create, delete, echo_ws, update, user_by_id, user_types, users};
use apistos::web;

use apistos::app::{BuildConfig, OpenApiWrapper};
use apistos::info::Info;
use apistos::server::Server;
use apistos::spec::Spec;
use apistos::{RapidocConfig, RedocConfig, ScalarConfig, SwaggerUIConfig};
use tracing::info;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //use tracing_subscriber;
    //tracing_subscriber::fmt().init();
    trace::init_tracing();

    info!(key = "value", "Hello");

    HttpServer::new(move || {
        let spec = Spec {
          info: Info {
            title: "A well documented API".to_string(),
            description: Some(
              "This is an API documented using Apistos,\na wonderful new tool to document your actix API !".to_string(),
            ),
            ..Default::default()
          },
          servers: vec![Server {
            url: "/".to_string(),
            ..Default::default()
          }],
          ..Default::default()
        };

        App::new()
            .document(spec)
            // .service(actix_web::web::resource("/ws").route(actix_web::web::get().to(echo_ws)))
            // .service(user_types)
            // .service(user_by_id)
            // .service(create)
            // .service(update)
            // .service(delete)
            // .service(users)
            .service(web::resource("/users/{type}").route(web::get().to(users)))
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .build_with(
              "/openapi.json",
              BuildConfig::default()
                .with(RapidocConfig::new(&"/rapidoc"))
                .with(RedocConfig::new(&"/redoc"))
                .with(ScalarConfig::new(&"/scalar"))
                .with(SwaggerUIConfig::new(&"/swagger")),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
