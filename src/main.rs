use actix_web::{web, App, HttpServer};
mod controllers {
    pub mod welcome;
}
mod types {
    pub mod welcome;
}
mod routes {
    pub mod user;
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
