use super::models::{User, UserInfo};
use actix_web::{delete, get, post, put, web, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct UserPath {
    r#type: String,
    id: u32,
}

#[get("/user-types")]
pub async fn user_types() -> Result<impl Responder> {
    let types = ["a", "b"];
    Ok(web::Json(types))
}

#[get("/users/{type}/{id}")]
pub async fn user_by_id(path: web::Path<UserPath>) -> Result<web::Json<User>> {
    let user = User {
        id: path.id.to_owned(),
        r#type: path.r#type.clone(),
        name: "aName".to_owned(),
    };
    Ok(web::Json(user))
}

#[delete("/users/{type}/{id}")]
pub async fn delete(path: web::Path<UserPath>) -> Result<impl Responder> {
    let user = User {
        id: path.id.to_owned(),
        r#type: path.r#type.clone(),
        name: "aName".to_owned(),
    };
    Ok(web::Json(user))
}

// #[tracing::instrument(fields(request_id = Uuid::new_v4().to_string()))]
#[get("/users/{type}")]
pub async fn users(r#type: web::Path<String>) -> Result<impl Responder> {
    let users = [
        User {
            id: 1,
            r#type: r#type.clone(),
            name: "aName".to_owned(),
        },
        User {
            id: 2,
            r#type: r#type.clone(),
            name: "anotherName".to_owned(),
        },
    ];

    Ok(web::Json(users))
}

#[post("/users/{type}")]
pub async fn create(
    r#type: web::Path<String>,
    user_info: web::Json<UserInfo>,
) -> Result<impl Responder> {
    let user = User {
        id: 1,
        r#type: r#type.clone(),
        name: user_info.name.clone(),
    };
    Ok(web::Json(user))
}

#[put("/users/{type}/{id}")]
pub async fn update(
    path: web::Path<UserPath>,
    user_update: web::Json<UserInfo>,
) -> Result<impl Responder> {
    let user = User {
        id: path.id.to_owned(),
        r#type: path.r#type.clone(),
        name: user_update.name.clone(),
    };
    Ok(web::Json(user))
}
