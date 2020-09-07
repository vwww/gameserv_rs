use actix_web::{dev::HttpServiceFactory, web, App, HttpResponse, HttpServer};
use std::env;

mod common;
mod duel;
mod slime;

/*
fn slime_scope() -> impl HttpServiceFactory {
    web::scope("")
        .service(web::resource("").to(|| HttpResponse::Ok().body("TODO1")))
        .service(web::resource("/n").to(|| HttpResponse::Ok().body("TODO2")))
}

fn duel_scope() -> impl HttpServiceFactory {
    web::scope("")
        .service(web::resource("").to(|| HttpResponse::Ok().body("TODO3")))
        .service(web::resource("/n").to(|| HttpResponse::Ok().body("TODO4")))
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind = match env::var("PORT") {
        Ok(port) => format!("[::]:{}", port),
        _ => "[::]:8080".to_owned(),
    };

    HttpServer::new(move || {
        App::new()
            // .service(web::scope("/s").service(slime_scope()))
            // .service(web::scope("/d").service(duel_scope()))
            .default_service(web::to(|| HttpResponse::Ok().body("hi")))
    })
    .bind(bind)?
    .run()
    .await
}
