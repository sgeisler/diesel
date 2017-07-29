extern crate manual_schema_and_mappings;
extern crate diesel;

use self::diesel::prelude::*;
use self::manual_schema_and_mappings::*;
use std::env::args;

fn main() {
    use manual_schema_and_mappings::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
