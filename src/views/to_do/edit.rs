use crate::database::DB;
use crate::diesel;
use diesel::prelude::*;

use rocket::serde::json::Json;
use crate::jwt::JwToken;

use crate::{
    json_serialization::{to_do_item::TodoItem, to_do_items::TodoItems},
    schema::to_do
};

#[post("/edit", data="<to_do_item>", format="json")]
pub async fn edit(to_do_item: Json<TodoItem>, token: JwToken, db: DB) -> Json<TodoItems> {
    let results = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title))
        .filter(to_do::columns::user_id.eq(&token.user_id));
    let _= diesel::update(results)
        .set(to_do::columns::status.eq("DONE"))
        .execute(&db.connection);
    Json(TodoItems::get_state(token.user_id))
}
