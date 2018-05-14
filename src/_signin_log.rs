use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::signin_log;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(SqlType)]
#[postgres(type_name = "user_role")]
pub struct User_role;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "User_role"]
pub enum User_role_enum {
    Customer,
    Supplier,
}

impl ToSql<User_role, Pg> for User_role_enum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            User_role_enum::Customer => out.write_all(b"customer")?,
            User_role_enum::Supplier => out.write_all(b"supplier")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<User_role, Pg> for User_role_enum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"customer" => Ok(User_role_enum::Customer),
            b"supplier" => Ok(User_role_enum::Supplier),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}