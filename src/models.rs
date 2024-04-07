use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "tb_user")]
pub struct UserFromDb{
    pub user_id: i32,
    pub user_login: String,
    pub user_password: String,
    pub user_role_id: i32,
    pub user_email: String
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "tb_role")]
pub struct RoleFromDb {
    pub role_id: i32,
    pub role_name: String
}

#[derive(Deserialize, Serialize)]
pub struct User{
    pub user_id: i32,
    pub user_login: String,
    pub user_password: String,
    pub user_role: String,
    pub user_email: String
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub user_login: String,
    pub user_password: String
}

#[derive(Deserialize, Serialize)]
pub struct UserRegister {
    pub user_login: String,
    pub user_password: String,
    pub user_email: String
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "tb_item")]
pub struct ItemFromDb {
    pub item_id: i32,
    pub item_name: String,
    pub item_description: String,
    pub item_type_id: i32,
    pub item_image: String
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "tb_item_type")]
pub struct ItemType {
    pub type_id: i32,
    pub type_name: String
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub item_id: i32,
    pub item_name: String,
    pub item_description: String,
    pub item_type: String,
    pub item_image: String
}

#[derive(Deserialize, Serialize)]
pub struct ItemInfo {
    pub item_name: String,
    pub item_description: String,
    pub item_type: String
}