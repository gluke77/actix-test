use actix_web::{dev::ServiceRequest, error, get, web, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use derive_more::derive::Constructor;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Admin,
    User,
}

#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct AuthUser {
    pub sub: String,
    exp: usize,
    pub permissions: Vec<Role>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct AuthToken {
    pub token: String,
}

pub async fn token_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (error::Error, ServiceRequest)> {
    info!("{credentials:?}");

    let token = decode::<AuthUser>(
        credentials.token(),
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    );

    match token {
        Err(err) => Err((error::ErrorUnauthorized(err), req)),
        Ok(TokenData { claims, .. }) => {
            req.extensions_mut().insert(claims);
            req.extensions_mut()
                .insert(AuthToken::new(credentials.token().to_owned()));

            Ok(req)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Constructor)]
struct Token {
    access_token: String,
}

#[get("/login")]
pub async fn login() -> Result<web::Json<Token>, actix_web::Error> {
    let exp = std::time::SystemTime::now()
        .checked_add(std::time::Duration::from_secs(60))
        .ok_or(error::ErrorInternalServerError("error calculating exp"))?
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| error::ErrorInternalServerError(e))?
        .as_secs();

    let user = AuthUser::new(
        "user".to_owned(),
        exp as usize,
        vec![Role::Admin, Role::User],
    );
    let access_token = encode(
        &Header::default(),
        &user,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(web::Json(Token::new(access_token)))
}

#[get("/verify")]
pub async fn verify(user: web::ReqData<AuthUser>) -> web::Json<AuthUser> {
    web::Json(user.into_inner())
}
