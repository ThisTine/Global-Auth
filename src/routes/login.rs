use actix_web::{http::header::ContentType,get,web,cookie, HttpResponse,post};
use serde::Deserialize;


#[derive(Deserialize)]
// #[derive(Debug)]
pub struct Request {
    user_name: String,
    pwd: String
}

#[get("/login/{user_name}/{pwd}")]
pub async fn login(info: web::Path<Request>)-> HttpResponse{
    let ck = cookie::Cookie::build("test","1234").finish();
    print!("{} {}",info.user_name,info.pwd);
    HttpResponse::Ok().content_type(ContentType::plaintext()).cookie(ck).body("test")
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String
}
#[post("/login")]
pub async fn login_post(info: web::Json<LoginRequest>)-> HttpResponse{
    HttpResponse::Ok().body(format!("{} {}",info.username,info.password))
}