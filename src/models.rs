use crate::errors::AppError;
use crate::schema::{users};
use diesel::prelude::*;
type Result<T> = std::result::Result<T, AppError>;

#[derive(Queryable, Identifiable, Serialize, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
}


pub fn create_user(conn: &SqliteConnection, username: &str) -> Result<User> {
    conn.transaction(|| {
        diesel::insert_into(users::table)
            .values((users::username.eq(username),))       
            .execute(conn)?;

        users::table
            .order(users::id.desc())
            .select((users::id, users::username))
            .first(conn)
            .map_err(Into::into)
    })
}

pub enum UserKey<'a> {
    Username(&'a str),
    ID(i32),
}


pub fn find_user<'a>(conn: &SqliteConnection, key: UserKey<'a>) -> Result<User> {

}