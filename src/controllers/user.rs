use crate::services;
use crate::types;
use actix_web::{web, Result};

pub async fn user_all() -> Result<web::Json<types::user::IUserAll>> {
  let ret = services::user::user_all().await;
  Ok(web::Json(ret))
}

// pub async fn user_one() -> Result<web::Json<types::user::IUserOne>> {
//   let ret = services::user::user_one().await;
//   Ok(web::Json(ret))
// }
