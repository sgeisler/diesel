extern crate manual_schema_and_mappings;
extern crate diesel;

use self::diesel::prelude::*;
use self::manual_schema_and_mappings::*;
use std::env::args;

fn main() {
    use manual_schema_and_mappings::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let _ = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(&connection)
        .expect(&format!("Unable to find post {}", id));

    let post: models::Post = posts.find(id)
        .first(&connection)
        .expect(&format!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}
