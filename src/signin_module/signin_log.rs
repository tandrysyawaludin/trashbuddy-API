use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use database::schema::signin_log;
use std::time::{SystemTime};

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(SqlType)]
#[postgres(type_name = "user_role")]
pub struct User_role;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "User_role"]
pub enum UserRoleEnum {
    Customer,
    Supplier
}

impl ToSql<User_role, Pg> for UserRoleEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            UserRoleEnum::Customer => out.write_all(b"customer")?,
            UserRoleEnum::Supplier => out.write_all(b"signin_log")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<User_role, Pg> for UserRoleEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"customer" => Ok(UserRoleEnum::Customer),
            b"signin_log" => Ok(UserRoleEnum::Supplier),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Insertable, Queryable, Debug, PartialEq)]
#[table_name = "signin_log"]
pub struct NewSigninLog {
    pub user_id: i32,
    pub user_group: UserRoleEnum,
    // pub optional_data: String
}

#[derive(Insertable, Queryable, Identifiable, Debug, PartialEq)]
#[table_name = "signin_log"]
pub struct SigninLog {
    pub id: i32,
    pub user_id: i32,
    pub user_group: UserRoleEnum,
    pub created_at: Option<SystemTime>,
    pub optional_data: Option<String>
}

#[derive(Insertable, Queryable, Debug, PartialEq)]
#[table_name = "signin_log"]
pub struct AlreadySigninLog {
    pub user_id: i32,
    pub user_group: UserRoleEnum,
    pub created_at: Option<SystemTime>,
    // pub optional_data: String
}

impl SigninLog {
    pub fn create(new_signin_log: NewSigninLog, connection: &PgConnection) -> SigninLog {
        diesel::insert_into(signin_log::table)
            .values(&new_signin_log)
            .execute(connection)
            .expect("Error creating new signin_log");

        signin_log::table.order(signin_log::id.desc()).first(connection).unwrap()
    }
    
    pub fn read(connection: &PgConnection) -> Vec<SigninLog> {
        signin_log::table.order(signin_log::id).load::<SigninLog>(connection).unwrap()
    }

    pub fn update(id: i32, signin_log: AlreadySigninLog, connection: &PgConnection) -> bool {
        diesel::update(signin_log::table.find(id)).set(&signin_log).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(signin_log::table.find(id)).execute(connection).is_ok()
    }
}
