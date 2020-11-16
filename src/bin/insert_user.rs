extern crate diesel;
extern crate orm_diesel;

use self::orm_diesel::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    let mut name = String::new();
    let mut email = String::new();
    let mut age = String::new();

    println!("Add a new user!");

    println!("Name: ");
    stdin().read_line(&mut name).unwrap();

    println!("Email: ");
    stdin().read_line(&mut email).unwrap();

    println!("Age: ");
    stdin().read_line(&mut age).unwrap();

    name = name.trim_end().to_string();
    email = email.trim_end().to_string();
    age = age.trim_end().to_string();

    create_user(&connection, &name, &email, &age.parse::<i32>().unwrap());
    println!("User {} saved.", &name);
}
