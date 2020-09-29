use actix_web::{middleware::Logger, web, App, HttpServer};

mod problem;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/problems").route(web::post().to(problem::solution)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
