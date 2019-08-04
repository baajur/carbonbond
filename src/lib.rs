pub mod api;
pub mod custom_error;
pub mod user;
pub mod forum;
pub mod db;
pub mod party;
pub mod config;

pub const MAX_ARTICLE_COLUMN: usize = 15;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate derive_more;
extern crate r2d2;
extern crate serde_json;
extern crate regex;
extern crate state;
extern crate actix;
extern crate actix_session;
extern crate actix_files;
extern crate chrono;

use actix_session::{Session};
use diesel::pg::PgConnection;
use crate::custom_error::{Error, Fallible};

pub trait Context {
    fn use_pg_conn<T, F>(&self, callback: F) -> Fallible<T>
    where
        F: FnOnce(PgConnection) -> Fallible<T>;
    fn remember_id(&self, id: i64) -> Fallible<()>;
    fn forget_id(&self) -> Fallible<()>;
    fn get_id(&self) -> Option<i64>;
}

pub struct Ctx {
    session: Session,
}
impl Ctx {
    pub fn get_pg_conn(&self) -> Fallible<PgConnection> {
        db::connect_db()
    }
}
impl Context for Ctx {
    fn use_pg_conn<T, F>(&self, callback: F) -> Fallible<T>
    where
        F: FnOnce(PgConnection) -> Fallible<T>,
    {
        let conn = db::connect_db()?;
        let result = callback(conn)?;
        Ok(result)
    }

    fn remember_id(&self, id: i64) -> Fallible<()> {
        self.session
            .set::<i64>("id", id)
            .map_err(|_| Error::new_internal_without_source("記憶 ID 失敗"))
    }

    fn forget_id(&self) -> Fallible<()> {
        self.session.remove("id");
        Ok(())
    }

    fn get_id(&self) -> Option<i64> {
        match self.session.get::<i64>("id") {
            Err(_) => {
                // TODO: 發生錯誤時隱性地清除 ID
                self.session.remove("id");
                None
            }
            Ok(result) => result,
        }
    }
}
