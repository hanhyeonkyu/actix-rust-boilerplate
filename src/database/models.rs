use super::schema::users;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: i32,
    pub email: String,
    pub pwd: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModUser {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub pwd: String,
}
