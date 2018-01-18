extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_demo::schema::robots::dsl::*;

    let connection = establish_connection();
    let results = robots.filter(published.eq(true))
        .limit(5)
        .load::<Robot>(&connection)
        .expect("Error loading robots");

    println!("Displaying {} robots", results.len());
    for post in results {
        println!("{}", robot.name);
        println!("----------\n");
        println!("{}", robot.avatar);
    }
}
