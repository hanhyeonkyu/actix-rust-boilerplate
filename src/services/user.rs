use crate::database::*;
use crate::schema::users::dsl::*;
use crate::types;
use actix_web::web;
use diesel::SqliteConnection;

use diesel::prelude::*;
use uuid::Uuid;

use models::*;
type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn user_all(conn: &mut SqliteConnection) -> Result<Option<Vec<models::User>>, DbError> {
    let ret = users
        .order(id.desc())
        .load::<models::User>(conn)
        .optional()?;
    Ok(ret)
}

pub fn find_user_by_uid(
    conn: &mut SqliteConnection,
    uid: String,
) -> Result<Option<models::User>, DbError> {
    let user = users
        .filter(id.eq(uid))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

pub fn make_user(
    conn: &mut SqliteConnection,
    body: web::Json<types::user::IMakeUserReq>,
) -> Result<Option<models::User>, DbError> {
    let new_user = User {
        id: Uuid::new_v4().to_string(),
        name: body.name.to_string(),
        age: body.age,
        email: body.email.to_string(),
        pwd: body.pwd.to_string(),
    };
    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(Some(new_user))
}

pub fn change_user(
    conn: &mut SqliteConnection,
    uid: String,
    body: web::Json<types::user::IChangeUserReq>,
) -> Result<Option<models::ModUser>, DbError> {
    let mod_user = ModUser {
        name: body.name.to_string(),
        age: body.age,
        email: body.email.to_string(),
        pwd: body.pwd.to_string(),
    };

    diesel::update(users.filter(id.eq(uid)))
        .set((
            name.eq(body.name.to_string()),
            age.eq(body.age),
            email.eq(body.email.to_string()),
            pwd.eq(body.pwd.to_string()),
        ))
        .execute(conn)
        .optional()?;

    Ok(Some(mod_user))
}
