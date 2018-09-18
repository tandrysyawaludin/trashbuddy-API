use database::schema::signin_logs;
use diesel;
use diesel::dsl::count;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;
use frank_jwt::{Algorithm, decode};

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "signin_logs"]
pub struct NewSigninLog {
  pub user_id: i32,
  pub token: String,
  pub is_valid: bool  
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "signin_logs"]
pub struct SigninLog {
  pub id: i32,
  pub user_id: i32,
  pub token: String,
  pub is_valid: bool,  
  pub created_at: Option<SystemTime>
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "signin_logs"]
pub struct AlreadySigninLog {
  pub is_valid: bool,
}

pub struct JWTContent {
  pub id: i32,
  pub email: String
}

impl SigninLog {
  pub fn create(new_signin_log: NewSigninLog, connection: &PgConnection) -> bool {
    diesel::insert_into(signin_logs::table)
      .values(&new_signin_log)
      .execute(connection)
      .is_ok()
  }

  pub fn read_after_create(connection: &PgConnection) -> SigninLog {
    signin_logs::table
      .order(signin_logs::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn update(token: String, signin_log: AlreadySigninLog, connection: &PgConnection) -> bool {
    let user_id = SigninLog::decode_jwt_get_id(token.to_string());
    let exists = signin_logs::table.filter(signin_logs::user_id.is_not_distinct_from(user_id)).limit(1).execute(connection);
    match exists {
      Ok(1) => diesel::update(signin_logs::table.filter(signin_logs::user_id.is_not_distinct_from(user_id)))
        .set(&signin_log)
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    diesel::delete(signin_logs::table.find(id))
      .execute(connection)
      .is_ok()
  }

  pub fn read(page: i64, connection: &PgConnection) -> Vec<SigninLog> {
    signin_logs::table
      .order(signin_logs::id)
      .limit(10)
      .offset(page * 10)
      .load::<SigninLog>(connection)
      .unwrap()
  }

  pub fn count_all(connection: &PgConnection) -> i64 {
    let total = signin_logs::table
      .select(count(signin_logs::id))
      .first::<i64>(connection);
    match total {
      Ok(v) => return v,
      Err(_e) => return 0,
    }
  }

  pub fn read_one(token: String, connection: &PgConnection) -> Vec<SigninLog> {
    let user_id = SigninLog::decode_jwt_get_id(token.to_string());    
    signin_logs::table
      .filter(signin_logs::user_id.is_not_distinct_from(user_id))
      .limit(1)
      .load::<SigninLog>(connection)
      .unwrap()
  }

  pub fn decode_jwt_get_id(header: String) -> i32 {
    let secret = "secret123";    
    let jwt = decode(&header.to_string(), &secret.to_string(), Algorithm::HS256);
    match jwt {
      Ok((header, payload)) => {
        let id: i32 = payload[0]["id"].to_string().parse().unwrap();
        return id
      },
      Err(e) => {
        return 0
      }
    }
  }
}
