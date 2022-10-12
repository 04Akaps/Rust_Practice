use actix_web::{get, web, HttpResponse, Responder, Result};
use mysql::prelude::*;
use mysql::*;
use serde::Serialize;

#[path = "../utils/util.rs"]
mod utils;
use utils::*;

#[derive(Debug, Serialize)]
struct MyData {
    id: u64,
    name: String,
    address: String,
    age: String,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/selectAll")]
async fn select_all() -> Result<impl Responder> {
    let mut conn = connect_to_mysql();
    let query = "select * from myData";

    let result = conn
        .query_map(query, |(id, name, address, age)| MyData {
            id: id,
            name: name,
            address: address,
            age: age,
        })
        .expect("쿼리 실패");

    println!("{:#?}", result);

    Ok(web::Json(result))
}
