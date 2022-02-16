pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

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
