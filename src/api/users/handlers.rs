use super::models::{User, UserInfo};
use actix_web::{delete, get, post, put, web};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Debug)]
struct UserPath {
    r#type: String,
    id: u32,
}

#[get("/user-types")]
pub async fn user_types() -> web::Json<Vec<String>> {
    let types = ["a".to_owned(), "b".to_owned()];
    nested().await;
    web::Json(types.to_vec())
}

#[tracing::instrument]
async fn nested() {
    deeper_nested().await
}

#[tracing::instrument]
async fn deeper_nested() {
    info!(key = "value", "From deeper_nested");
}

#[get("/users/{type}/{id}")]
pub async fn user_by_id(path: web::Path<UserPath>) -> web::Json<User> {
    let user = User {
        id: path.id.to_owned(),
        r#type: path.r#type.clone(),
        name: "aName".to_owned(),
    };
    web::Json(user)
}

#[delete("/users/{type}/{id}")]
pub async fn delete(path: web::Path<UserPath>) -> web::Json<User> {
    let user = User {
        id: path.id.to_owned(),
        r#type: path.r#type.clone(),
        name: "aName".to_owned(),
    };
    web::Json(user)
}

#[get("/users/{type}")]
pub async fn users(r#type: web::Path<String>) -> web::Json<Vec<User>> {
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

    info!("I can log");
    info!("So do I");

    web::Json(users.to_vec())
}

#[post("/users/{type}")]
pub async fn create(r#type: web::Path<String>, user_info: web::Json<UserInfo>) -> web::Json<User> {
    let user = User {
        id: 1,
        r#type: r#type.clone(),
        name: user_info.name.clone(),
    };
    web::Json(user)
}

#[put("/users/{type}/{id}")]
pub async fn update(
    path: web::Path<UserPath>,
    user_update: web::Json<UserInfo>,
) -> web::Json<User> {
    let user = User {
        id: path.id.to_owned(),
        r#type: path.r#type.clone(),
        name: user_update.name.clone(),
    };
    web::Json(user)
}
