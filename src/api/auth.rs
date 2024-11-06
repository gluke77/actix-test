use actix_web::{dev::ServiceRequest, error, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct AuthUser {
    pub user: String,
    pub permissions: Vec<String>,
}

pub async fn token_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    info!("{credentials:?}");

    if credentials.token().contains('x') {
        return Err((error::ErrorUnauthorized("bad token"), req));
    }

    req.extensions_mut().insert(AuthUser::new(
        "Test user".to_owned(),
        vec!["user".to_owned(), "admin".to_owned()],
    ));

    Ok(req)
}
