use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::db::models::User;
use crate::db::schema;
use crate::custom_error::{Error, Fallible};

pub fn find_user(conn: &PgConnection, id: i64) -> Fallible<User> {
    // TODO: 對 SQL 查詢的錯誤做分類
    schema::users::table
        .find(id)
        .first::<User>(conn)
        .map_err(|_| Error::new_logic(format!("找不到使用者: {}", id), 401))
}
