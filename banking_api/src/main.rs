use std::sync::Mutex;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>
}
#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello {}!", app_name))
}

#[get("/counter")]
async fn counter(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("Request Number {}!", *counter))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {


   HttpServer::new(|| {
       App::new()
           .app_data(web::Data::new(AppState {
               app_name: String::from("banking"),

           }))
           .service(hello)
   })
       .bind("127.0.0.1:8080")?
       .run()
       .await
}
