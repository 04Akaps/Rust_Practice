use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpServer};
use std::sync::Mutex;

mod api;
mod utils;

use api::get_api::manual_hello;

use api::post_api::echo;

use utils::util::*;

struct AppStateWithCounter {
    app_name: String,
    counter: Mutex<i32>,
}

#[get("/")]
async fn start(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    let app_name = &data.app_name;

    *counter += 1;

    format!("appName is : {app_name} Request number: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let status = web::Data::new(AppStateWithCounter {
        app_name: "my first rust server".to_string(),
        counter: Mutex::new(0),
    });
    connect_to_mysql();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://www.rust-lang.org/")
            .allowed_methods(vec!["GET,POST,DELETE"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(status.clone())
            .service(start)
            .service(web::scope("/app").service(echo).service(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
