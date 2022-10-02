use crate::{database::user_model::ModUser, user_model::User};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IUserAll {
    pub rt: bool,
    pub dt: Option<Vec<User>>,
    pub mg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IUserOne {
    pub rt: bool,
    pub dt: Option<User>,
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
    pub dt: Option<User>,
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
    pub dt: Option<ModUser>,
    pub mg: String,
}
