use actix_web::{ get, web, App,  HttpServer, HttpResponse};
use std::sync::Mutex;

struct AppStateWithCounter {
    app_name: String,
    counter : Mutex<i32>
}

mod api;
mod utils;

use api::get_api::{
    manual_hello,
};

use api::post_api::{
    echo
};

#[get("/")]
async fn start(data : web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); 
    let app_name = &data.app_name;

    *counter += 1; 

    format!("appName is : {app_name} Request number: {counter}")
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
    // router를 이렇게도 작성이 가능합니다.
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let status = web::Data::new(AppStateWithCounter {
        app_name : "my first rust server".to_string(),
        counter : Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
        .app_data(status.clone()) 
        .configure(config)
        .service(start)
        .service(web::scope("/app").service(echo).service(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await


}
