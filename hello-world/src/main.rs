use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

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

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .service(hello)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
            // .service(web::scope("/app").route("/index.html", web::get().to(index)))
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
