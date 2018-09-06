use database::schema::suppliers;
use diesel;
use diesel::dsl::count;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;
use frank_jwt::{Algorithm, encode, decode};
use djangohashers::{check_password, make_password};

#[table_name = "suppliers"]
#[derive(Serialize, Deserialize, Insertable, Debug)]
pub struct NewSupplier {
  pub name: String,
  pub email: String,
  pub password: String,
  pub phone_number: String,
  pub area: String,
}

#[table_name = "suppliers"]
#[derive(Serialize, Deserialize, Queryable, AsChangeset, Debug)]
pub struct Supplier {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub password: String,
  pub phone_number: String,
  pub area: String,
  pub created_at: Option<SystemTime>,
}

#[table_name = "suppliers"]
#[derive(Serialize, Deserialize, Queryable, AsChangeset, Debug)]
pub struct AlreadySupplier {
  pub name: String,
  pub email: String,
  pub password: String,
  pub phone_number: String,
  pub area: String,
  pub created_at: Option<SystemTime>,
}

#[table_name = "suppliers"]
#[derive(Serialize, Deserialize, Queryable, AsChangeset, Debug)]
pub struct AuthSupplier {
  pub email: String,
  pub password: String  
}

#[table_name = "suppliers"]
#[derive(Serialize, Deserialize, Queryable, AsChangeset, Debug)]
pub struct JWTContentSupplier {
  pub id: i32,
  pub email: String
}

impl Supplier {
  pub fn create(mut new_supplier: NewSupplier, connection: &PgConnection) -> bool {
    let encoded_password = make_password(&new_supplier.password);
    new_supplier.password = encoded_password.to_string();
    diesel::insert_into(suppliers::table)
      .values(&new_supplier)
      .execute(connection)
      .is_ok()
  }

  pub fn read_after_create(connection: &PgConnection) -> Supplier {
    suppliers::table
      .order(suppliers::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn update(id: i32, supplier: AlreadySupplier, connection: &PgConnection) -> bool {
    let exists = suppliers::table.find(id).limit(1).execute(connection);
    match exists {
      Ok(1) => diesel::update(suppliers::table.find(id))
        .set(&supplier)
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    let exists = suppliers::table.find(id).limit(1).execute(connection);
    match exists {
      Ok(1) => diesel::delete(suppliers::table.find(id))
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn read(page: i64, connection: &PgConnection) -> Vec<Supplier> {
    suppliers::table
      .order(suppliers::id)
      .limit(10)
      .offset(page * 10)
      .load::<Supplier>(connection)
      .unwrap()
  }

  pub fn count_all(connection: &PgConnection) -> i64 {
    let total = suppliers::table
      .select(count(suppliers::id))
      .first::<i64>(connection);
    match total {
      Ok(v) => return v,
      Err(_e) => return 0,
    }
  }

  pub fn read_one(id: i32, connection: &PgConnection) -> Vec<Supplier> {
    suppliers::table
      .find(id)
      .limit(1)
      .load::<Supplier>(connection)
      .unwrap()
  }

  pub fn auth(data: AuthSupplier, connection: &PgConnection) -> bool {
    let exists = suppliers::table
      .select(suppliers::password)    
      .filter(suppliers::email.is_not_distinct_from(data.email))
      .limit(1)
      .first::<String>(connection);
    match exists {
      Ok(v) => {
        return check_password(&data.password, &v).unwrap()
      },
      _ => return false,
    }
  }

  pub fn read_jwt(email: String, connection: &PgConnection) -> Vec<JWTContentSupplier> {
    suppliers::table
      .select((suppliers::id, suppliers::email))
      .filter(suppliers::email.is_not_distinct_from(email))
      .load::<JWTContentSupplier>(connection)
      .unwrap()
  }

  pub fn encode_jwt(supplier: Vec<JWTContentSupplier>, connection: &PgConnection) -> String {
    let payload = json_internal!(supplier);
    let header = json_internal!({});
    let secret = "secret123";
    let jwt = encode(header, &secret.to_string(), &payload, Algorithm::HS256);
    match jwt {
      Ok(v) => return v,
      Err(e) => return "".to_string(),
    }
  }

  pub fn decode_jwt(header: String, connection: &PgConnection) -> Vec<JWTContentSupplier> {
    let secret = "secret123";    
    let jwt = decode(&header.to_string(), &secret.to_string(), Algorithm::HS256);
    match jwt {
      Ok((header, payload)) => {
        let id: i32 = payload[0]["id"].to_string().parse().unwrap();
        let email = payload[0]["email"].to_string().trim_matches('\"').to_string();
        let decode_jwt = vec![JWTContentSupplier{id: id, email: email}];
        return decode_jwt
      },
      Err(e) => {
        return Vec::new()
      }
    }
  }
}
