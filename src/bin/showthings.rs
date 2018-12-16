extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_demo::schema::things::dsl::*;
    let connection = establish_connection();
    let results = things.filter(active.eq(true))
        .limit(5)
        .order(id.asc())
        .load::<Thing>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{} {}", post.id, post.name);
        println!("----{}", post.description);
    }
}
