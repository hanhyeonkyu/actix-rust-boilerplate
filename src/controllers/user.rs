use crate::interfaces;
use crate::interfaces::global::TDbPool;
use crate::interfaces::user::IChangeUserRes;
use crate::interfaces::user::IMakeUserRes;
use crate::interfaces::user::IUserAll;
use crate::interfaces::user::IUserOne;
use crate::services;
use actix_web::{web, Result};
use uuid::Uuid;

pub async fn user_all(pool: web::Data<TDbPool>) -> Result<web::Json<interfaces::user::IUserAll>> {
    println!("Failed to perform necessary steps: {}", "test");
    let users = web::block(move || {
        let mut conn = pool.get()?;
        services::user::user_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(users) = users {
        Ok(web::Json(IUserAll {
            rt: true,
            dt: Some(users),
            mg: "Success".to_string(),
        }))
    } else {
        Ok(web::Json(IUserAll {
            rt: false,
            dt: users,
            mg: "Fail".to_string(),
        }))
    }
}

pub async fn user_one(
    pool: web::Data<TDbPool>,
    user_uid: web::Path<Uuid>,
) -> Result<web::Json<interfaces::user::IUserOne>> {
    let user_uid = user_uid.into_inner();
    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || {
        let mut conn = pool.get()?;
        services::user::find_user_by_uid(&mut conn, user_uid.to_string())
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(user) = user {
        Ok(web::Json(IUserOne {
            rt: true,
            dt: Some(user),
            mg: "Success".to_string(),
        }))
    } else {
        Ok(web::Json(IUserOne {
            rt: false,
            dt: user,
            mg: format!("No user found with uid: {user_uid}"),
        }))
    }
}

pub async fn make_user(
    pool: web::Data<TDbPool>,
    body: web::Json<interfaces::user::IMakeUserReq>,
) -> Result<web::Json<interfaces::user::IMakeUserRes>> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        services::user::make_user(&mut conn, body)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(user) = user {
        Ok(web::Json(IMakeUserRes {
            rt: true,
            dt: Some(user),
            mg: "Success".to_string(),
        }))
    } else {
        Ok(web::Json(IMakeUserRes {
            rt: false,
            dt: user,
            mg: "Fail".to_string(),
        }))
    }
}

pub async fn change_user(
    pool: web::Data<TDbPool>,
    user_uid: web::Path<Uuid>,
    body: web::Json<interfaces::user::IChangeUserReq>,
) -> Result<web::Json<interfaces::user::IChangeUserRes>> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        services::user::change_user(&mut conn, user_uid.to_string(), body)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(user) = user {
        Ok(web::Json(IChangeUserRes {
            rt: true,
            dt: Some(user),
            mg: "Success".to_string(),
        }))
    } else {
        Ok(web::Json(IChangeUserRes {
            rt: false,
            dt: user,
            mg: "Fail".to_string(),
        }))
    }
}
