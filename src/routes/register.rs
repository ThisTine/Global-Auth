use actix_web::{post,web, HttpResponse};
use bcrypt::DEFAULT_COST;
use sea_orm::{ActiveValue, ActiveModelTrait};
use serde::{Deserialize,Serialize};


use crate::AppState;
use crate::db::user;

#[derive(Deserialize)]
struct RegisterInfo {
    email: String,
    username: String,
    password: String
}

#[derive(Serialize)]
struct RegisterResponse {
    id: String,
    email: String,
    username: String,
}

#[post("/register")]
async fn register(data: web::Data<AppState>,info: web::Json<RegisterInfo>)-> HttpResponse{
    println!("{}",info.email);
    let conn = &data.conn;
    let password = bcrypt::hash(info.password.to_owned(), DEFAULT_COST);
    let usr = user::ActiveModel{
        // id: ActiveValue::set(Uuid::new_v4()),
        email: ActiveValue::Set(info.email.to_owned()),
        password: ActiveValue::Set(password.unwrap()),
        username: ActiveValue::Set(info.username.to_owned()),
        ..Default::default()
    };
    let res: user::Model = usr.insert(conn).await.unwrap();

    let response = RegisterResponse {
        email: res.email,
        id: res.id.to_string(),
        username: res.username
    };

    // Ok(web::Json(response))
    HttpResponse::Ok().json(web::Json(response))
}