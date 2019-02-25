extern crate diesel_demo;
extern crate diesel;


use self::diesel::prelude::*;
use self::diesel_demo::*;
use std::env::args;
use self::models::*;

fn main() {
    use diesel_demo::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();

    let get_result = |in_str: &str| -> Result<Post, String> {
        let results = posts.filter(title.like(in_str))
            .limit(1)
            .load::<Post>(&connection)
            .expect("Error loading posts");
        let d = Box::new(results);
        if (&d).len() > 0 {
            let a = d.to_vec();
            let b = &a[0];
            let c = b.clone();
            Ok(c)
        }
        else {
            Err("Strange".to_string())
        }
    };

    let result = match get_result(&pattern)  {
        Ok(post) => {
            println!("{}", post.title);
            println!("------\n");
            println!("{}", post.body);            
        }
        Err(err) => {
            println!("error {}", err);
        }
    };

}
