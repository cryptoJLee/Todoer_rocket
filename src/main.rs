#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
use diesel::prelude::*;

use rocket::serde::json::Json;

mod schema;
mod database;
mod json_serialization;
mod models;
mod to_do;
mod config;

use crate::models::item::item::Item;
use crate::json_serialization::to_do_items::TodoItems;
use crate::models::item::new_item::NewItem;
use database::DBCONNECTION;

#[post("/create/<title>")]
fn item_create(title: String) -> Json<TodoItems> {
    let db = DBCONNECTION.db_connection.get().unwrap();
    let items = schema::to_do::table
        .filter(schema::to_do::columns::title.eq(&title.as_str()))
        .order(schema::to_do::columns::id.asc())
        .load::<Item>(&db)
        .unwrap();
    if items.len() == 0 {
        let new_post = NewItem::new(title, 1);
        let _ = diesel::insert_into(schema::to_do::table)
            .values(&new_post)
            .execute(&db);
    }
    Json(TodoItems::get_state(1))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
        .mount("/v1/item", routes![item_create])
}