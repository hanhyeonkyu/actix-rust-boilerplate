pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

use models::*;
use schema::users;
use schema::users::dsl::*;

fn establish_connection() -> SqliteConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  SqliteConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_users() -> Vec<User> {
  let connection = establish_connection();
  users
    .load::<User>(&connection)
    .expect("Error loading users")
}

pub fn get_user(search_id: String) -> User {
  let connection = establish_connection();
  users
    .find(search_id)
    .first(&connection)
    .expect("Error loading users")
}

pub fn create_user(name: &str, age: &i32, email: &str, pwd: &str) -> String {
  let connection = establish_connection();
  let uuid = Uuid::new_v4().to_hyphenated().to_string();
  let new_user = vec![NewUser {
    id: &uuid,
    name: name,
    age: age,
    email: email,
    pwd: pwd,
  }];
  diesel::insert_into(users::table)
    .values(&new_user)
    .execute(&connection)
    .expect("Error create new user");
  uuid
}
