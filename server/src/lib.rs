#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate validator_derive;

mod auth;
mod config;
mod db;
mod errors;
mod models;
mod routes;
mod schema;

use dotenv::dotenv;
use rocket_contrib::json::JsonValue;
use rocket_cors;
use rocket_cors::Cors;

pub fn build_rocket() -> rocket::Rocket {
    dotenv().ok();

    let config = config::from_env();
    rocket::custom(config)
        .mount(
            "/api",
            routes![
                routes::users::get_user,
                routes::users::post_users,
                routes::users::post_login,
                routes::users::put_user,
            ],
        )
        .attach(db::Conn::fairing())
        .attach(cors_fairing())
        .attach(config::AppState::manage())
}

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing failed to be created")
}

