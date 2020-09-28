use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/problems").route(web::post().to_async(submit_problem)))
        .service(web::resource("/problems/{id}/solutions").route(web::get().to_async(get_solutions)));
}

async fn submit_problem(path: web::Path<String>) -> HttpResponse {

}

async fn get_solutions(path: web::Path<String>) -> HttpResponse {

}