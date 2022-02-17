use crate::services;
use crate::types;
use actix_web::{web, Result};

pub async fn user_all() -> Result<web::Json<types::user::IUserAll>> {
  let ret = services::user::user_all().await;
  Ok(web::Json(ret))
}

pub async fn user_one(id: web::Path<String>) -> Result<web::Json<types::user::IUserOne>> {
  let ret = services::user::user_one(id.to_string()).await;
  Ok(web::Json(ret))
}

pub async fn make_user(
  body: web::Json<types::user::IMakeUserReq>,
) -> Result<web::Json<types::user::IMakeUserRes>> {
  let ret = services::user::make_user(body).await;
  Ok(web::Json(ret))
}

pub async fn change_user(
  id: web::Path<String>,
  body: web::Json<types::user::IChangeUserReq>,
) -> Result<web::Json<types::user::IChangeUserRes>> {
  let ret = services::user::change_user(id.to_string(), body).await;
  Ok(web::Json(ret))
}
