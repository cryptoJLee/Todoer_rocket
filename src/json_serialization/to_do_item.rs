use serde::Deserialize;

#[derive(Deserialize)]
pub struct TodoItem {
  pub title: String,
  pub status: String
}