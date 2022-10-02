pub mod user_model;
pub mod user_schema;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

pub fn establish_connection(conn_spec: String) -> Pool<ConnectionManager<SqliteConnection>> {
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    return pool;
}
