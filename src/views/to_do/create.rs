use crate::diesel;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::response::status::Created;

use crate::json_serialization::to_do_items::TodoItems;
use crate::database::DB;
use crate::models::item::new_item::NewItem;
use crate::models::item::item::Item;
use crate::schema::to_do;
use crate::jwt::JwToken;

#[post("/create/<title>")]
pub async fn create<'a>(title: String, db: DB, token: JwToken) -> Created<Json<TodoItems>> {
  let items = to_do::table
      .filter(to_do::columns::title.eq(&title.as_str()))
      .filter(to_do::columns::user_id.eq(&token.user_id))
      .order(to_do::columns::id.asc())
      .load::<Item>(&db.connection)
      .unwrap();
  if items.len() == 0 {
    let new_post = NewItem::new(title, token.user_id);
    let _ = 
        diesel::insert_into(to_do::table).values(&new_post)
        .execute(&db.connection);
  }
  let body = Json(TodoItems::get_state(token.user_id));
  Created::new("").body(body)
}
