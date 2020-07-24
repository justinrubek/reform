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
                routes::schemas::get_schema,
                routes::schemas::get_all_schemas,
                routes::schemas::post_schema,
                routes::schemas::get_schema_entries,
                routes::entries::get_entry,
                routes::entries::post_entries,
                routes::entries::put_entry,
            ],
        )
        .attach(db::Conn::fairing())
        .attach(cors_fairing())
        .attach(config::AppState::manage())
}

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing failed to be created")
}

