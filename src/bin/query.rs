use self::models::*;
use diesel::prelude::*;
use orm_diesel::*;

fn main() {
    use self::schema::Users::dsl::*;

    let connection = establish_connection();

    let users = Users
        .filter(age.eq(18))
        .load::<User>(&connection)
        .expect("Error loading users");


    if users.len() > 0 {
        println!("All 18 years old users:");

        for user in users {
            println!("{}", user.name);
        }
    } else {
        println!("There aren't 18 years old users");
    }
}