use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};

use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::Utc;

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwToken {
  pub user_id: i32,
  pub exp: usize,
}

impl JwToken {
  pub fn get_key() -> String {
    let config = Config::new();
    let key_str = config.map.get("SECRET_KEY").unwrap().as_str().unwrap();
    key_str.to_owned()
  }

  pub fn encode(self) -> String {
    let key = EncodingKey::from_secret(JwToken::get_key().as_ref());
    let token = encode(&Header::default(), &self, &key).unwrap();
    token
  }

  pub fn new(user_id: i32) -> Self {
    let config = Config::new();
    let minutes = config.map.get("EXPIRE_MINUTES")
      .unwrap().as_i64().unwrap();
    let expiration = Utc::now()
      .checked_add_signed(chrono::Duration::minutes(minutes))
      .expect("valid timestamp")
      .timestamp();
    JwToken { user_id, exp: expiration as usize }
  }

  pub fn from_token(token: String) -> Result<Self, String> {
    let key = DecodingKey::from_secret(JwToken::get_key().as_ref());
    let token_result = decode::<JwToken>(
      &token, &key, &Validation::default()
    );
    match token_result {
      Ok(data) => Ok(data.claims),
      Err(error) => {
        let message = format!("{}", error);
        return Err(message)
      }
    }
  }
}

#[derive(Debug)]
pub enum JwTokenError {
  Missing,
  Invalid,
  Expired
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwToken {
  type Error = JwTokenError;
  
  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    match req.headers().get_one("token") {
      Some(data) => {
        let raw_token = data.to_string();
        let token_result = JwToken::from_token(raw_token);
        match token_result {
          Ok(token) => Outcome::Success(token),
          Err(message) => {
            if message == "ExpiredSignature".to_owned() {
              return Outcome::Failure((Status::BadRequest, JwTokenError::Expired))
            }
            Outcome::Failure((Status::BadRequest, JwTokenError::Invalid))
          }
        }
      },
      None => {
        Outcome::Failure((Status::BadRequest, JwTokenError::Missing))
      }
    }
  }
}

#[cfg(test)]
mod jwt_tests {
  use std::str::FromStr;

  use super::{JwToken, Config};
  use serde_json::json;
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Serialize, Deserialize)]
  pub struct ResponseFromTest {
    pub user_id: i32,
    pub exp_minutes: i32,
  }

  #[test]
  fn get_key() {
    assert_eq!(String::from("secret"), JwToken::get_key());
  }

  #[test]
  fn get_exp() {
    let config = Config::new();
    let minutes = config.map.get("EXPIRE_MINUTES").unwrap().as_i64().unwrap();
    assert_eq!(120, minutes);
  }

  #[test]
  fn decode_incorrect_token() {
    let encoded_token: String = String::from("invalid_token");
    match JwToken::from_token(encoded_token) {
      Err(message) => assert_eq!("InvalidToken", message),
      _ => panic!("Incorrect token should not be able to encoded")
    }
  }

  #[test]
  fn encode_decode() {
    let test_token = JwToken::new(5);
    let encoded_token = test_token.encode();
    let new_token = JwToken::from_token(encoded_token).unwrap();
    assert_eq!(5, new_token.user_id);
  }

  // async fn test_handler(token:JwToken, _:HttpRequest) -> HttpResponse {
  //   HttpResponse::Ok().json(json!({
  //     "user_id": token.user_id,
  //     "exp_minutes": 60
  //   }))
  // }

  // #[actix_web::test]
  // async fn test_no_token_request() {
  //   let app = init_service(App::new().route("/", web::get().to(test_handler))).await;
  //   let req = TestRequest::default().insert_header(ContentType::plaintext()).to_request();
  //   let resp = call_service(&app, req).await;
  //   assert_eq!("401", resp.status().as_str());
  // }

  // #[actix_web::test]
  // async fn test_passing_token_request() {
  //   let test_token = JwToken::new(5);
  //   let encoded_token = test_token.encode();
  //   let app = init_service(App::new().route("/", web::get().to(test_handler))).await;
  //   let mut req = TestRequest::default().insert_header(ContentType::plaintext()).to_request();
  //   let header_name = HeaderName::from_str("token").unwrap();
  //   let header_value = HeaderValue::from_str(encoded_token.as_str()).unwrap();
  //   req.headers_mut().insert(header_name, header_value);
  //   let resp: ResponseFromTest = actix_web::test::call_and_read_body_json(&app, req).await;
  //   assert_eq!(5, resp.user_id);
  // }

  // #[actix_web::test]
  // async fn test_false_token_request() {
  //   let app = init_service(App::new().route("/", web::get().to(test_handler))).await;
  //   let mut req = TestRequest::default().insert_header(ContentType::plaintext()).to_request();
  //   let header_name = HeaderName::from_str("token").unwrap();
  //   let header_value = HeaderValue::from_str("test").unwrap();
  //   req.headers_mut().insert(header_name, header_value);
  //   let resp = call_service(&app, req).await;
  //   assert_eq!("401", resp.status().as_str());
  // }
}