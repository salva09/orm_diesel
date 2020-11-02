extern crate diesel;
extern crate orm_diesel;

use self::models::*;
use diesel::prelude::*;
use orm_diesel::*;

fn main() {
    use self::schema::Users::dsl::*;

    let connection = establish_connection();
    let results = Users
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("-----------");
        println!("{}", user.name);
        println!("{}", user.email);
        println!("{} years old", user.age);
        println!("-----------");
    }
}