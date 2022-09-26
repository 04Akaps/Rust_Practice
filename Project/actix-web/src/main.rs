use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod api;

use api::get_api::{
    hello,
    manual_hello
};

use api::post_api::{
    echo
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server Started At : {}", 8080);
    
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/app").service(echo).service(manual_hello))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
