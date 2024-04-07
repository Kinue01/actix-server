use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{ errors::MyError, models::{ UserFromDb, UserRegister, ItemFromDb, User, RoleFromDb, ItemType, Item, ItemInfo } };

pub async fn get_users(client: &Client) -> Result<Vec<User>, MyError> {

    let result = client.query("select * from tb_user order by user_id", &[]).await?
    .iter().map(|row| UserFromDb::from_row_ref(row).unwrap()).collect::<Vec<UserFromDb>>();

    let roles = client.query("select * from tb_role", &[]).await?
    .iter().map(|row| RoleFromDb::from_row_ref(row).unwrap()).collect::<Vec<RoleFromDb>>();

    let mut users: Vec<User> = Vec::new();

    for user in &result {
        users.push(
            User 
            { 
                user_id: user.user_id, 
                user_login: user.user_login.clone(), 
                user_password: user.user_password.clone(), 
                user_role: roles.iter().find(|&role| role.role_id == user.user_role_id).unwrap().role_name.clone(), 
                user_email: user.user_email.clone() 
            }
        )
    }

    Ok(users)
}

pub async fn add_user(client: &Client, user: UserRegister) -> Result<(), MyError> {
    
    client.execute("insert into tb_user (user_login, user_password, user_email) values ($1, $2, $3)", &[&user.user_login, &user.user_password, &user.user_email]).await?;

    Ok(())
}

pub async fn get_items(client: &Client) -> Result<Vec<Item>, MyError> {

    let res = client.query("select * from tb_item order by item_id", &[]).await?
    .iter().map(|row| ItemFromDb::from_row_ref(row).unwrap()).collect::<Vec<ItemFromDb>>();

    let types = client.query("select * from tb_item_type", &[]).await?
    .iter().map(|row| ItemType::from_row_ref(row).unwrap()).collect::<Vec<ItemType>>();

    let mut items: Vec<Item> = Vec::new();

    for item in &res {
        items.push
        (
            Item 
            { 
                item_id: item.item_id, 
                item_name: item.item_name.clone(), 
                item_description: item.item_description.clone(), 
                item_type: types.iter().find(|&typ| typ.type_id == item.item_type_id ).unwrap().type_name.clone(),
                item_image: item.item_image.clone()
            }
        )
    }

    Ok(items)
}

pub async fn add_item(client: &Client, item: ItemInfo) -> Result<(), MyError> {
    
    let res = client.query("select * from tb_item_type", &[]).await?
    .iter().map(|row| ItemType::from_row_ref(row).unwrap()).collect::<Vec<ItemType>>();

    client.execute("insert into tb_item (item_name, item_description, item_type_id, item_image) values ($1, $2, $3, '@/assets/default.png')", &[&item.item_name, &item.item_description, &res.iter().find(|itm| itm.type_name == item.item_type).unwrap().type_id]).await?;

    Ok(())
}

pub async fn get_item_types(client: &Client) -> Result<Vec<ItemType>, MyError> {

    let types = client.query("select * from tb_item_type", &[]).await?
    .iter().map(|row| ItemType::from_row_ref(row).unwrap()).collect::<Vec<ItemType>>();

    Ok(types)
}