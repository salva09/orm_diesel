use self::models::*;
use diesel::prelude::*;
use orm_diesel::*;
use std::io::stdin;

fn main() {
    use self::schema::Users::dsl::*;

    let mut user_id = String::new();

    println!("Enter the user id to delete: ");
    stdin()
        .read_line(&mut user_id)
        .expect("Failed to read input");
    let user_id = user_id.trim_end().parse::<i32>().expect("Id not valid");

    let connection = establish_connection();

    let success = diesel::delete(Users.find(user_id))
        .execute(&connection)
        .unwrap();

    if success == 1 {
        println!("The user {} was deleted successfully", user_id);
    } else {
        println!("User {} could not be deleted", user_id);
    }
}
