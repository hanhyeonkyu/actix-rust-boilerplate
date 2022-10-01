use crate::controllers;
use actix_web::web;

pub fn user_routes() -> actix_web::Scope {
    return web::scope("/user")
        .route(
            "/all/{offset}/{limit}",
            web::get().to(controllers::user::user_all),
        )
        .route("/{user_uid}", web::get().to(controllers::user::user_one))
        .route("/post", web::post().to(controllers::user::make_user))
        .route(
            "/{user_uid}/put",
            web::put().to(controllers::user::change_user),
        );
}
