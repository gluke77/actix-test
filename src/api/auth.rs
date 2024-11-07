use actix_web::{
    dev::ServiceRequest,
    error, get,
    web::{self, Data},
    HttpMessage,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use derive_more::derive::Constructor;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::AppState;

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

pub async fn token_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (error::Error, ServiceRequest)> {
    info!("{credentials:?}");

    let key = &req.app_data::<Data<AppState>>().unwrap().keys.decoding_key;

    let token = decode::<AuthUser>(
        credentials.token(),
        &DecodingKey::from_rsa_der(key),
        &Validation::new(jsonwebtoken::Algorithm::RS256),
    );

    match token {
        Err(err) => Err((error::ErrorUnauthorized(err), req)),
        Ok(TokenData { claims, .. }) => {
            req.extensions_mut().insert(claims);

            Ok(req)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Constructor)]
struct Token {
    access_token: String,
}

#[get("/login")]
pub async fn login(data: web::Data<AppState>) -> Result<web::Json<Token>, actix_web::Error> {
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
        &Header::new(jsonwebtoken::Algorithm::RS256),
        &user,
        &EncodingKey::from_rsa_der(&data.keys.encoding_key),
    )
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(web::Json(Token::new(access_token)))
}

#[get("/verify")]
pub async fn verify(user: web::ReqData<AuthUser>) -> web::Json<AuthUser> {
    web::Json(user.into_inner())
}

#[derive(Clone)]
pub struct Keys {
    pub encoding_key: Vec<u8>,
    pub decoding_key: Vec<u8>,
}

pub fn generate_keys() -> Keys {
    let mut rng = rand::thread_rng();
    let private_key = rsa::RsaPrivateKey::new(&mut rng, 4096).expect("failed to generate a key");
    let public_key = rsa::RsaPublicKey::from(&private_key);

    let enkey = private_key.to_pkcs1_der().unwrap().as_bytes().to_vec();
    let dekey = public_key.to_pkcs1_der().unwrap().as_bytes().to_vec();

    Keys {
        encoding_key: enkey,
        decoding_key: dekey,
    }
}
