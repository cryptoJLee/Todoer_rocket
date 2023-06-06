#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
use diesel::prelude::*;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

mod schema;
mod database;
mod json_serialization;
mod models;
mod to_do;
mod jwt;
mod config;
mod views;

use views::auth::{login::login, login::login_get, logout::logout};
use views::to_do::{create::create, delete::delete, edit::edit, get::get};
use views::users::create::create_user;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
                    .mount("/v1/item/", routes![create, delete, get, edit])
                    .mount("/v1/auth/", routes![login, login_get, logout])
                    .mount("/v1/user/", routes![create_user])
                    .attach(CORS)
                    .manage(CORS)
}