use actix_web::{ web, Error, HttpResponse, post, get };
use deadpool_postgres::{ Client, Pool };

use crate::{db, errors::MyError, models::{UserInfo, UserRegister, ItemInfo} };

#[get("/api/users")]
pub async fn get_users(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let users = db::get_users(&client).await?;

    Ok(HttpResponse::Ok().json(users))
}

#[post("/api/adduser")]
pub async fn add_user(user: web::Json<UserRegister>, db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {

    let usr_info = user.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    db::add_user(&client, usr_info).await?;

    Ok(HttpResponse::Created().finish())
}

#[post("/api/getuser")]
pub async fn get_user(user: web::Json<UserInfo>, db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {

    let usr_info = user.into_inner();
    
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let users = db::get_users(&client).await?;

    let is_user: Vec<_> = users.iter()
    .filter(|usr| usr.user_login == usr_info.user_login && usr.user_password == usr_info.user_password).collect();

    Ok(HttpResponse::Ok().json(is_user))
}

#[get("/api/getitems")]
pub async fn get_items(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let items = db::get_items(&client).await?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("/api/additem")]
pub async fn add_item(item: web::Json<ItemInfo>, db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {

    let itm_info = item.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    db::add_item(&client, itm_info).await?;

    Ok(HttpResponse::Created().finish())
}

#[get("/api/getitemtypes")]
pub async fn get_item_types(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let types = db::get_item_types(&client).await?;

    Ok(HttpResponse::Ok().json(types))

}