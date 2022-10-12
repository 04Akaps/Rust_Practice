use actix_web::{error, get, post, web, Error, HttpResponse, Responder, Result};
use futures::StreamExt;
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};

#[path = "../utils/util.rs"]
mod utils;
use utils::get_db_object;

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    name: String,
    address: String,
    age: String,
}

const MAX_SIZE: usize = 262_144;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/selectAll")]
async fn select_all() -> Result<impl Responder> {
    let mut conn = get_db_object();
    let query = "SELECT name, address, age from myData";

    let result = conn
        .query_map(query, |(name, address, age)| MyData { name, address, age })
        .expect("쿼리 실패");

    println!("get select All from MyData");

    Ok(web::Json(result))
}

#[post("/addMyData")]
async fn addMyData(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // https://docs.rs/actix-web/4.2.1/actix_web/web/struct.Payload.html

    println!("New POST request to create a post!");

    let mut body = web::BytesMut::new();

    //  데이터를 payload stream으로 모으는 while문
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let obj: MyData = serde_json::from_slice::<MyData>(&body).expect("실패....");

    println!("저장된 데이터는 이와 같습니다. {:#?}", obj);

    let mut conn = get_db_object();

    let payments = vec![MyData {
        name: obj.name,
        address: obj.address,
        age: obj.age,
    }];

    conn.exec_batch(
        r"INSERT INTO myData (name, address, age) VALUES (:name, :address, :age)",
        payments.iter().map(|p| {
            params! {
                "name" => &p.name,
                "address" => &p.address,
                "age" => &p.age
            }
        }),
    )
    .expect("무슨 코드지 이게..");

    Ok(HttpResponse::Ok().json("success")) // <- send response
}
