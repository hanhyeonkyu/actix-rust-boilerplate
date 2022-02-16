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
