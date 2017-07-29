extern crate manual_schema_and_mappings;
extern crate diesel;

use self::manual_schema_and_mappings::*;
use self::manual_schema_and_mappings::models::*;
use self::diesel::prelude::*;

fn main() {
    use manual_schema_and_mappings::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
