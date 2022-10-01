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
    pub mod global;
    pub mod user;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // set up database connection pool
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let pool = establish_connection(conn_spec);

    let ldp = "127.0.0.1:8080";
    log::info!("{}", format!("Server running at: http://{}", ldp));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(controllers::welcome::landing))
            .route("/echo", web::post().to(controllers::welcome::echo))
            .service(routes::user::user_routes())
    })
    .bind(ldp)?
    .run()
    .await
}
