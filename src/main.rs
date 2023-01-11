use actix_web::{HttpServer, App,get,HttpResponse,Responder,web};

mod routes;
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use routes::{login,register};
mod db;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Debug,Clone)]
pub struct AppState {
    conn: DatabaseConnection
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {

    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    println!("{}",db_url);

    let conn = Database::connect(&db_url).await.unwrap();

    let state = AppState{conn};

    
    HttpServer::new(move|| {
        App::new()
        .app_data(web::Data::new(state.clone()))
        .service(hello)
        .service(login::login)
        .service(login::login_post)
        .service(register::register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}   
