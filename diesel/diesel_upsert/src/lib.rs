#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use chrono::{Duration, NaiveDateTime, Utc};

pub mod schema;
pub mod models;

use self::models::{User, NewUser};
use crate::schema::users::columns::name;
use crate::schema::users::columns::hair_color;


pub fn establish_connection() -> PgConnection{
	dotenv().ok();

	let database_url = env::var("DATABASE_URL")
		.expect("DATABASE_URL must be set");

	PgConnection::establish(&database_url)
		.expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(conn: &PgConnection, namestr: &'a str, hair_colorstr: &'a str, minutes_to_expire: i64){
	use schema::users;

	let now = Utc::now().naive_utc();
	let expiration_time = now + Duration::minutes(minutes_to_expire);

	let new_user = NewUser {
		name: namestr, 
		hair_color: hair_colorstr,
		expiration_time
	};

	diesel::insert_into(users::table)
		.values(&new_user)
		.on_conflict(name)
		.do_update()
		.set(&new_user)
		.execute(conn).unwrap()
	;
}

pub fn verify_user<'a>(conn: &PgConnection, namestr: &'a str, hair_colorstr: &'a str) {
	use crate::schema::users::dsl::*;

	let get_results = |in_str: &str| -> Result<User, String> {
		let results = users.filter(name.eq(in_str))
			.limit(1)
			.load::<User>(conn)
			.expect("Error loading user");

		let d = Box::new(results);
		if (&d).len() > 0{
			let a = d.to_vec();
			let b = &a[0];
			let c = b.clone();
			Ok(c)
		}
		else {
			Err("Strange".to_string())
		}
		
	};

	match get_results(&namestr) {
		Ok(user) =>{
			let now = Utc::now().naive_utc().timestamp();
			let exp = user.expiration_time.timestamp();
			if exp < now{
				println!("expired")
			}
			else if hair_colorstr != user.hair_color{
				println!("unmatch")
			}
			else {
				println!("match")
			}

		},
		Err(err) => {
			println!("error {}", err);
		}
			
	};

	
}
