use crate::controllers;
use actix_web::web;

pub fn user_routes() -> actix_web::Scope {
  return web::scope("/user")
    .route("/all", web::get().to(controllers::user::user_all))
    .route("/{id}", web::get().to(controllers::user::user_one));
}
