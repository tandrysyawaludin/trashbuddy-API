use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::signin_log;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(SqlType)]
#[postgres(type_name = "status_transaction")]
pub struct Status_transaction;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "Status_transaction"]
pub enum Status_transaction_enum {
    Pending,
    Process,
    Success,
}

impl ToSql<Status_transaction, Pg> for Status_transaction_enum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            Status_transaction_enum::Pending => out.write_all(b"pending")?,
            Status_transaction_enum::Process => out.write_all(b"process")?,
            Status_transaction_enum::Success => out.write_all(b"success")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Status_transaction, Pg> for Status_transaction_enum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"pending" => Ok(Status_transaction_enum::Pending),
            b"process" => Ok(Status_transaction_enum::Process),
            b"success" => Ok(Status_transaction_enum::Success),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}