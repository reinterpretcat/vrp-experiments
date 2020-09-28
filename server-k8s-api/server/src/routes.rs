use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/problems").route(web::post().to_async(solve_problem)));
}

async fn solve_problem(_path: web::Path<String>) -> HttpResponse {
    unimplemented!()
}
