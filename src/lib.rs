extern crate mysql;

pub use mysql::Value;
pub use mysql::Row;
pub use mysql::error::Error;

#[macro_use]
pub mod macros;
pub mod cond;
pub mod db;

use db::DB;

pub trait Entity {
    fn get_create_table() -> String;
    fn get_table() -> String;
    fn get_fields() -> String;
    fn get_prepare() -> String;
    fn set_id(&mut self, id: u64);
    fn get_id_cond(&self) -> String;
    fn get_params(&self) -> Vec<(String, Value)>;
    fn get_params_id(&self) -> Vec<(String, Value)>;
    fn from_row(mut row: mysql::conn::Row) -> Self;
}

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
