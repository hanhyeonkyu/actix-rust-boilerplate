use super::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
pub struct User {
  pub id: String,
  pub name: String,
  pub age: i32,
  pub email: String,
  pub pwd: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub id: &'a str,
  pub name: &'a str,
  pub age: &'a i32,
  pub email: &'a str,
  pub pwd: &'a str,
}
