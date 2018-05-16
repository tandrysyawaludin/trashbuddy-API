use database::schema::signin_log;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "signin_log"]
pub struct NewSigninLog {
  pub user_id: i32,
  pub user_group: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "signin_log"]
pub struct SigninLog {
  pub id: i32,
  pub user_id: i32,
  pub user_group: String,
  pub created_at: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "signin_log"]
pub struct AlreadySigninLog {
  pub user_id: i32,
  pub user_group: String,
  pub created_at: Option<SystemTime>,
}

impl SigninLog {
  pub fn create(new_signin_log: NewSigninLog, connection: &PgConnection) -> SigninLog {
    diesel::insert_into(signin_log::table)
      .values(&new_signin_log)
      .execute(connection)
      .expect("Error creating new signin_log");

    signin_log::table
      .order(signin_log::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn read(connection: &PgConnection) -> Vec<SigninLog> {
    signin_log::table
      .order(signin_log::id)
      .load::<SigninLog>(connection)
      .unwrap()
  }

  pub fn update(id: i32, signin_log: AlreadySigninLog, connection: &PgConnection) -> bool {
    diesel::update(signin_log::table.find(id))
      .set(&signin_log)
      .execute(connection)
      .is_ok()
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    diesel::delete(signin_log::table.find(id))
      .execute(connection)
      .is_ok()
  }
}
