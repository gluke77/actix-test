use crate::AppState;

use actix_web::{error, get, web, Error, HttpRequest};

use actix_web_httpauth::extractors::bearer::BearerAuth;
use tracing::info;

#[get("/parent")]
pub async fn parent(
    http_request: HttpRequest,
    auth: BearerAuth,
    data: web::Data<AppState>,
) -> Result<String, Error> {
    let headers = http_request.headers();

    // Iterate over the headers and print each header name and value
    for (name, value) in headers.iter() {
        info!(
            "Header: {:?} = {:?}",
            name,
            value.to_str().unwrap_or("Invalid UTF-8")
        );
    }

    let uri = format!("http://localhost:{}/v1/child", data.child_port);
    data.client
        .get(uri)
        .bearer_auth(auth.token())
        .send()
        .await
        .map_err(|e| error::ErrorInternalServerError(e))?
        .text()
        .await
        .map_err(|e| error::ErrorInternalServerError(e))
}

#[get("/child")]
pub async fn child(http_request: HttpRequest) -> &'static str {
    let headers = http_request.headers();

    // Iterate over the headers and print each header name and value
    for (name, value) in headers.iter() {
        info!(
            "Header: {:?} = {:?}",
            name,
            value.to_str().unwrap_or("Invalid UTF-8")
        );
    }

    "I'm child"
}
