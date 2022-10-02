use diesel::{
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct IEcho {
    pub name: String,
}

pub type TDbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
