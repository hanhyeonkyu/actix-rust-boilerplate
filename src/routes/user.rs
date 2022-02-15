use crate::controllers;
use actix_web::web;

pub fn user_routes() -> actix_web::Scope {
  return web::scope("/user")
    .route("", web::get().to(controllers::welcome::landing))
    .route("/echo", web::post().to(controllers::welcome::echo));
}
