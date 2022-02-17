use crate::models::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IUserAll {
  pub rt: bool,
  pub dt: Vec<User>,
  pub mg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IUserOne {
  pub rt: bool,
  pub dt: User,
  pub mg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IMakeUserReq {
  pub name: String,
  pub age: i32,
  pub email: String,
  pub pwd: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IMakeUserRes {
  pub rt: bool,
  pub dt: String,
  pub mg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IChangeUserReq {
  pub name: String,
  pub age: i32,
  pub email: String,
  pub pwd: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IChangeUserRes {
  pub rt: bool,
  pub dt: usize,
  pub mg: String,
}
