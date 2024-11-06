use crate::{api::auth, AppState};

use actix_web::{get, web, HttpRequest};

use tracing::info;

#[get("/parent")]
pub async fn parent(http_request: HttpRequest, data: web::Data<AppState>) -> String {
    let headers = http_request.headers();

    // Iterate over the headers and print each header name and value
    for (name, value) in headers.iter() {
        info!(
            "Header: {:?} = {:?}",
            name,
            value.to_str().unwrap_or("Invalid UTF-8")
        );
    }

    let uri = format!("http://localhost:{}/child", data.child_port);
    data.client
        .get(uri)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

#[get("/child")]
pub async fn child(user: web::ReqData<auth::AuthUser>, http_request: HttpRequest) -> &'static str {
    let headers = http_request.headers();

    // Iterate over the headers and print each header name and value
    for (name, value) in headers.iter() {
        info!(
            "Header: {:?} = {:?}",
            name,
            value.to_str().unwrap_or("Invalid UTF-8")
        );
    }

    let u = user.into_inner();

    info!("User {:?}", &u);

    if u.permissions.contains(&"admin".to_owned()) {
        info!("admin");
    }

    "I'm child"
}
