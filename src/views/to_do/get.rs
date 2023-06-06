use rocket::serde::json::Json;

use crate::json_serialization::to_do_items::TodoItems;
use crate::jwt::JwToken;

#[get("/get")]
pub async fn get(token: JwToken) -> Json<TodoItems> {
  Json(TodoItems::get_state(token.user_id))
}