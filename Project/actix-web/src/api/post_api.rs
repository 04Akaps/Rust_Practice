use actix_web::{error, post, web, Error, HttpResponse};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    name: String,
    address: String,
    age: String,
}

const MAX_SIZE: usize = 262_144;

#[post("/addMyData")]
async fn echo(mut payload: web::Payload) -> Result<HttpResponse, Error> {
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

    println!("{:#?}", obj);

    Ok(HttpResponse::Ok().json(obj)) // <- send response
}
