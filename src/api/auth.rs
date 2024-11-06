use actix_web::{dev::ServiceRequest, error, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use derive_more::derive::Constructor;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct AuthUser {
    pub sub: String,
    pub exp: usize,
    pub permissions: Vec<String>,
}

pub async fn token_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    info!("{credentials:?}");

    let token = decode::<AuthUser>(
        &credentials.token(),
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    );

    match token {
        Err(err) => Err((error::ErrorUnauthorized(err), req)),
        Ok(TokenData { claims, .. }) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
    }
}
