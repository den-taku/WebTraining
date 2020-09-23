use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use thiserror::Error;

struct Null {}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemlate {
    entries: Vec<Null>,
}

// enum paymentstate {
//     already,
//     still,
// }

// struct MembershipEntry {
    // grade: u8,
    // name: String,
    // // state: bool, // already payed -> true
// }

// #[derive(Template)]
// #[template(path = "membership.html")]
// struct MembershipTemplate {
        // entries: Vec<MembershipEntry>,
// }

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}

impl ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let mut entries = Vec::new();
    entries.push(Null{});
    let html = IndexTemlate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

// #[get("/membership.html")]
// async fn membership() -> Result<HttpResponse, MyError> {
    // let mut entries = Vec::new();
    // entries.push(MembershipEntry{ 
        // grade: 3,
        // name: "DenTaku".to_string(),
        // // state: true,
    // });
    // let html = MembershipTemplate { entries };
    // let response_body = html.render()?;
    // Ok(HttpResponse::Ok()
        // .content_type("text/html")
        // .body(response_body))
// }

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
