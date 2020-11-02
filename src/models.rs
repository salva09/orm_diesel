use super::schema::Users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(Insertable)]
#[table_name="Users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub age: &'a i32,
}