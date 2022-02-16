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

// pub async fn user_one() -> user::IUserOne {
//   let usr1 = user::IUser {
//     id: 1,
//     name: "alex".to_string(),
//     age: 33,
//     email: "aiji.alexhan@gmail.com".to_string(),
//     pwd: "123qwe".to_string(),
//   };
//   let ret = user::IUserOne {
//     rt: true,
//     dt: usr1,
//     mg: "success".to_string(),
//   };
//   return ret;
// }
