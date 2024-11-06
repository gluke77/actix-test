use actix_web::{dev::ServiceRequest, error, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use tracing::info;

pub async fn token_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    info!("{credentials:?}");

    if credentials.token().contains('x') {
        return Err((error::ErrorUnauthorized("bad token"), req));
    }

    Ok(req)
}
