use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello wordl!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// async fn index() -> impl Responder {
//     "Hello world!"
// }

struct AppState {
    app_name: String,
}

// #[get("/")]
// async fn index(data: web::Data<AppState>) -> String {
//     let app_name = &data.app_name;
//     format!("Hello {}!", app_name)
// }

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request naumber: {}", counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("www"))),
            )
            .service(
                web::scope("/")
                .guard(guard::Header("Host", "users.rust-lang.org"))
                .route("", web::to(|| HttpResponse::Ok().body("user"))),
            )
            .route("/", web::to(|| HttpServer::Ok()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    //let counter = web::Data::new(AppStateWithCounter {
    //    counter: Mutex::new(0),
    //});
    // let scope = web::scope("/users").service(show_users);
    // App::new().service(scope);
    // HttpServer::new(move || {
        // App::new()
            // .service(hello)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
            // .service(web::scope("/app").route("/index.html", web::get().to(index)))
            // .data(AppState {
            //     app_name: String::from("Actix-web"),
            // })
            // .app_data(counter.clone())
            // .route("/", web::get().to(index))
            // .service(index)
    // })
    // .bind("127.0.0.1:8080")?
    // .run()
    // .await
}
