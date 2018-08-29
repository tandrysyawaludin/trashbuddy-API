use database::schema::partners;
use diesel;
use diesel::dsl::count;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;
use diesel::expression::dsl::sql;
use diesel::sql_types::Bool;
use rand::{ RngCore, Rng, OsRng };

// Crypto
#[path = "../crypto_module/mod.rs"] mod crypto_module;
// End Crypto

#[table_name = "partners"]
#[derive(Serialize, Deserialize, Insertable)]
pub struct NewPartner {
  pub name: String,
  pub password: String,
  pub phone_number: String,
  pub email: String,
  pub area: String,
  pub machine_code: String,
}

#[table_name = "partners"]
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Partner {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub password: String,
  pub phone_number: String,
  pub area: String,
  pub category_of_trash_id: Option<Vec<String>>,
  pub machine_code: Option<String>,
  pub is_live: bool,
  pub created_at: Option<SystemTime>,
}

#[table_name = "partners"]
#[derive(Serialize, Deserialize, Insertable)]
pub struct SignInPartner {
  pub email: String,
  pub password: String,
}

#[table_name = "partners"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct AlreadyPartner {
  pub name: String,
  pub password: String,
  pub phone_number: String,
  pub email: String,
  pub area: String,
  pub machine_code: String,
  pub created_at: Option<SystemTime>,
}

impl Partner {
  pub fn create(new_partner: NewPartner, connection: &PgConnection) -> bool {
    diesel::insert_into(partners::table)
      .values(&new_partner)
      .execute(connection)
      .is_ok()
  }

  pub fn read_after_create(connection: &PgConnection) -> Partner {
    partners::table
      .order(partners::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn update(id: i32, partner: AlreadyPartner, connection: &PgConnection) -> bool {
    let exists = partners::table.find(id).limit(1).execute(connection);
    match exists {
      Ok(1) => diesel::update(partners::table.find(id))
        .set(&partner)
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    let exists = partners::table.find(id).limit(1).execute(connection);
    match exists {
      Ok(1) => diesel::delete(partners::table.find(id))
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn read(page: i64, area: String, category: String, connection: &PgConnection) -> Vec<Partner> {
    let message = "Hello World!";
    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];
    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);

    let encrypted_data = crypto_module::encrypt(message.as_bytes(), &key, &iv).ok().unwrap();
    println!("bajing {:?}", encrypted_data);
    let decrypted_data = crypto_module::decrypt(&encrypted_data[..], &key, &iv).ok().unwrap();
    println!("jingtot {:?}", decrypted_data);
    
    partners::table
      .filter(partners::area.eq(&area))
      .filter(sql::<Bool>("category_of_trash_id @> ARRAY['s']::text[]"))
      .order(partners::id)
      .limit(10)
      .offset(page * 10)
      .load::<Partner>(connection)
      .unwrap()
  }

  pub fn count_all(connection: &PgConnection) -> i64 {
    let total = partners::table
      .select(count(partners::id))
      .first::<i64>(connection);
    match total {
      Ok(v) => return v,
      Err(_e) => return 0,
    }
  }

  pub fn read_one(id: i32, connection: &PgConnection) -> Vec<Partner> {
    partners::table
      .find(id)
      .limit(1)
      .load::<Partner>(connection)
      .unwrap()
  }

  pub fn sign_in(email: String, password: String, connection: &PgConnection) -> Vec<Partner> {
    partners::table
      .filter(partners::email.eq(&email))
      .filter(partners::password.eq(&password))
      .limit(1)
      .load::<Partner>(connection)
      .unwrap()
  }
}
