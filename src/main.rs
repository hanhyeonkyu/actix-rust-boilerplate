#[macro_use]
extern crate diesel;

extern crate dotenv;

use actix_web::{web, App, HttpServer};
mod controllers {
    pub mod user;
    pub mod welcome;
}
mod database;
use database::*;
mod routes {
    pub mod user;
}
mod services {
    pub mod user;
}
mod types {
    pub mod user;
    pub mod welcome;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ldp = "127.0.0.1:8080";
    println!("{}", format!("server running at: http://{}", ldp));
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(controllers::welcome::landing))
            .route("/echo", web::post().to(controllers::welcome::echo))
            .service(routes::user::user_routes())
    })
    .bind(ldp)?
    .run()
    .await
}
