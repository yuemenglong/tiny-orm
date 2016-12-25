use mysql::Pool;
use mysql::error::Error;
use std::cell::RefCell;

use cond::Cond;
use Entity;

pub struct DB {
    pub pool: Pool,
}

impl DB {
    pub fn create_table<E: Entity>(&self) -> Result<u64, Error> {
        let sql = E::get_create_table();
        println!("{}", sql);
        let res = self.pool.prep_exec(sql, ());
        match res {
            Ok(res) => Ok(res.affected_rows()),
            Err(err) => Err(err),
        }
    }
    pub fn drop_table<E: Entity>(&self) -> Result<u64, Error> {
        let sql = E::get_drop_table();
        println!("{}", sql);
        let res = self.pool.prep_exec(sql, ());
        match res {
            Ok(res) => Ok(res.affected_rows()),
            Err(err) => Err(err),
        }
    }
    pub fn insert<E: Entity + Clone>(&self, entity: &E) -> Result<E, Error> {
        let sql = format!("INSERT INTO `{}` SET {}", E::get_name(), E::get_prepare());
        println!("{}", sql);
        let res = self.pool.prep_exec(sql, entity.get_params());
        match res {
            Ok(res) => {
                let mut ret = (*entity).clone();
                ret.set_id(res.last_insert_id());
                Ok(ret)
            }
            Err(err) => Err(err),
        }
    }
    pub fn update<E: Entity>(&self, entity: &E) -> Result<u64, Error> {
        let sql = format!("UPDATE `{}` SET {} WHERE `id` = {}",
                          E::get_name(),
                          E::get_prepare(),
                          entity.get_id().unwrap());
        println!("{}", sql);
        let res = self.pool.prep_exec(sql, entity.get_params());
        match res {
            Ok(res) => Ok(res.affected_rows()),
            Err(err) => Err(err),
        }
    }
    pub fn get<E: Entity>(&self, id: u64) -> Result<Option<E>, Error> {
        let sql = format!("SELECT {} FROM `{}` WHERE `id` = {}",
                          E::get_field_list(),
                          E::get_name(),
                          id);
        println!("{}", sql);
        let res = self.pool.first_exec(sql, ());
        match res {
            Ok(option) => Ok(option.map(|row| E::from_row(row))),
            Err(err) => Err(err),
        }
    }
    pub fn delete<E: Entity>(&self, entity: E) -> Result<u64, Error> {
        let sql = format!("DELETE FROM `{}` WHERE `id` = {}",
                          E::get_name(),
                          entity.get_id().unwrap());
        println!("{}", sql);
        let res = self.pool.prep_exec(sql, ());
        match res {
            Ok(res) => Ok(res.affected_rows()),
            Err(err) => Err(err),
        }
    }
    pub fn select(&self, conds: Vec<Cond>) -> SelectBuilder {
        SelectBuilder {
            pool: &self.pool,
            conds: RefCell::new(conds),
        }
    }
}

pub struct SelectBuilder<'a> {
    pool: &'a Pool,
    conds: RefCell<Vec<Cond>>,
}

impl<'a> SelectBuilder<'a> {
    pub fn execute<E: Entity>(self) -> Result<Vec<E>, Error> {
        let cond_str = match self.conds.borrow().len() {
            0 => String::new(),
            _ => {
                let cb = |cond: &Cond| cond.to_prepare();
                let str = self.conds.borrow().iter().map(cb).collect::<Vec<_>>().join(" AND ");
                format!(" WHERE {}", str)
            }
        };
        let sql = format!("SELECT `{}` FROM {}{}",
                          E::get_field_list(),
                          E::get_name(),
                          cond_str);
        println!("{}", sql);
        let mut params = Vec::new();
        for cond in self.conds.into_inner().into_iter() {
            cond.to_param(&mut params);
        }
        let res = self.pool.prep_exec(sql, params);
        if res.is_err() {
            return Err(res.unwrap_err());
        }
        let res = res.unwrap();
        let vec = res.map(|row| E::from_row(row.unwrap())).collect();
        Ok(vec)
    }
    pub fn and(self, conds: Vec<Cond>) -> Self {
        self.conds.borrow_mut().extend(conds);
        self
    }
}
