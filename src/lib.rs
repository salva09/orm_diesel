#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::{User, NewUser};

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(conn: &SqliteConnection, name: &'a str, email: &'a str, age: &'a i32) -> usize {
    use schema::Users;

    let new_user = NewUser{
        name,
        email,
        age,
    };

    diesel::insert_into(Users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user")
}