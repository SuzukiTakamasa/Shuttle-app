
use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
        .route("", web::get().to(get_all)) // get all films data
        .route("/{film_id}", web::get().to(get)) // get films by id
        .route("", web::post().to(post)) //post new film
        .route("", web::put().to(put)) //update film
        .route("/{film_id}", web::delete().to(delete)), // delte film
    );
}

async fn get_all() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn post() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn put() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn delete() -> HttpResponse {
    HttpResponse::Ok().finish()
}