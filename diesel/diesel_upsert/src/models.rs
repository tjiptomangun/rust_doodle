use super::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
#[derive(Queryable, Clone)]
pub struct User {
	pub id: i64,
	pub title: String,
	pub hair_color: String,
	pub created: NaiveDateTime,
	pub expiration_time: NaiveDateTime,	
}

#[derive(Insertable, AsChangeset)]
#[table_name="users"]
pub struct NewUser<'a>{
	pub name: &'a str,
	pub hair_color: &'a str,
	pub expiration_time: NaiveDateTime,
}
