use crate::database::*;
use crate::models::*;
use crate::types;
use actix_web::web;

pub async fn user_all() -> types::user::IUserAll {
  let dt = get_users();
  let ret = types::user::IUserAll {
    rt: true,
    dt,
    mg: "success".to_string(),
  };
  return ret;
}

pub async fn user_one(id: String) -> types::user::IUserOne {
  let dt = get_user(id);
  let ret = types::user::IUserOne {
    rt: true,
    dt,
    mg: "success".to_string(),
  };
  return ret;
}

pub async fn make_user(body: web::Json<types::user::IMakeUserReq>) -> types::user::IMakeUserRes {
  let new_user = NewUser {
    id: "will be change uuid",
    name: &body.name,
    age: &body.age,
    email: &body.email,
    pwd: &body.pwd,
  };
  let dt = create_user(new_user);
  let ret = types::user::IMakeUserRes {
    rt: true,
    dt,
    mg: "success".to_string(),
  };
  return ret;
}

pub async fn change_user(
  id: String,
  body: web::Json<types::user::IChangeUserReq>,
) -> types::user::IChangeUserRes {
  let mod_user = NewUser {
    id: "will be change uuid",
    name: &body.name,
    age: &body.age,
    email: &body.email,
    pwd: &body.pwd,
  };
  let dt = update_user(id, mod_user);
  let ret = types::user::IChangeUserRes {
    rt: true,
    dt,
    mg: "success".to_string(),
  };
  return ret;
}
