use crate::database::*;
use crate::types;

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
