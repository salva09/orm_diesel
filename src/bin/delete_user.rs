use self::models::*;
use diesel::prelude::*;
use orm_diesel::*;
use std::io::stdin;

fn main() {
    use self::schema::Users::dsl::*;

    let mut wanted_id = String::new();

    println!("Enter the id to delete: ");
    stdin().read_line(&mut wanted_id).expect("Failed to read input");
    let wanted_id = wanted_id.trim_end().parse::<i32>().expect("Id not valid");

    let connection = establish_connection();

    let valid_id = Users
        .find(wanted_id)
        .load::<User>(&connection)
        .expect("Failed to select users")
        .len() == 1;

    if (valid_id) {
        diesel::delete(Users.find(wanted_id))
            .execute(&connection)
            .expect("Failed to delete user");

        println!("Deleted the user {} succesfully", wanted_id);
    } else {
        println!("Id not found.")
    }
}