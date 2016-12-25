extern crate mysql;
extern crate chrono;

pub use mysql::Row;
pub use std::collections::HashMap;
pub use mysql::Error;
pub use chrono::NaiveDate as Date;
pub use chrono::NaiveTime as Time;
pub use chrono::NaiveDateTime as DateTime;
pub use mysql::Value;

#[macro_use]
pub mod macros;
pub mod cond;
pub mod db;
pub mod entity;

pub use entity::Entity;
pub use entity::FieldMeta;
pub use cond::Cond;
pub use db::DB;

pub fn open(user: &str, pwd: &str, host: &str, port: u16, db: &str) -> Result<DB, Error> {
    let conn_str = format!("mysql://{}:{}@{}:{}/{}", user, pwd, host, port, db);
    match mysql::Pool::new(conn_str.as_ref()) {
        Ok(pool) => Ok(DB { pool: pool }),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
