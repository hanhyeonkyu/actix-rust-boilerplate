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

pub fn create_user(new_user: NewUser) -> String {
  let connection = establish_connection();
  let uuid = Uuid::new_v4().to_hyphenated().to_string();
  diesel::insert_into(users::table)
    .values((
      id.eq(&uuid),
      name.eq(new_user.name),
      age.eq(new_user.age),
      email.eq(new_user.email),
      pwd.eq(new_user.pwd),
    ))
    .execute(&connection)
    .expect("Error create new user");
  uuid
}

pub fn update_user(key: String, mod_user: NewUser) -> usize {
  let connection = establish_connection();
  diesel::update(users.find(key))
    .set((
      name.eq(mod_user.name),
      age.eq(mod_user.age),
      email.eq(mod_user.email),
      pwd.eq(mod_user.pwd),
    ))
    .execute(&connection)
    .expect("Can't Update User")
}
