use crate::types::welcome;
use actix_web::{web, Result};

pub async fn landing() -> Result<String> {
  Ok("Welcome Actix Web Landing Page".to_string())
}

pub async fn echo(echo: web::Json<welcome::IEcho>) -> Result<String> {
  Ok(format!("Welcome {}!", echo.name))
}
