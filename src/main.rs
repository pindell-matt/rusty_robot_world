#![feature(plugin)]
#![plugin(rocket_codegen)]

#![feature(plugin, custom_derive, const_fn)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

pub mod schema;

use diesel::pg::PgConnection;
use r2d2::{ Pool, Config };
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

use rocket::Rocket;

// can be moved to routes.rs
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn rocket() -> Rocket {
    dotenv().ok();

    // Url to the database as set in the DATABASE_URL environment variable.
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Initializes database pool with r2d2.
    let config = Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(config, manager).expect("Failed to create pool.");

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![index])
}

fn main() {
    rocket().launch();
}
