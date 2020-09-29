use actix_web::{middleware::Logger, web, App, HttpServer};

mod problem;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port = std::env::var("PORT")
        .map_err(|err| err.to_string())
        .and_then(|value| value.parse::<i32>().map_err(|err| err.to_string()))
        .unwrap_or_else(|err| panic!("cannot get port value from PORT: {}", err));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/problems").route(web::post().to(problem::solution)))
    })
    .bind(format!("127.0.0.1:{}", port).as_str())?
    .run()
    .await
}
