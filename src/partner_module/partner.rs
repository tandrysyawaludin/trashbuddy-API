use database::schema::partners;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;

#[table_name = "partners"]
#[derive(Serialize, Deserialize, Insertable)]
pub struct NewPartner {
  pub name: String,
  pub email: String,
  pub password: String,
  pub phone_number: String,
  pub area: String,
  pub machine_code: String,
}

#[table_name = "partners"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Partner {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub password: String,
  pub phone_number: String,
  pub area: String,
  pub machine_code: String,
  pub created_at: Option<SystemTime>,
}

#[table_name = "partners"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct AlreadyPartner {
  pub name: String,
  pub email: String,
  pub password: String,
  pub phone_number: String,
  pub area: String,
  pub machine_code: String,
  pub created_at: Option<SystemTime>,
}

impl Partner {
  pub fn create(new_partner: NewPartner, connection: &PgConnection) -> Partner {
    diesel::insert_into(partners::table)
      .values(&new_partner)
      .execute(connection)
      .expect("Error creating new partner");

    partners::table
      .order(partners::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn read(connection: &PgConnection) -> Vec<Partner> {
    partners::table
      .order(partners::id)
      .load::<Partner>(connection)
      .unwrap()
  }

  pub fn read_one(id: i32, connection: &PgConnection) -> Vec<Partner> {
    partners::table
      .find(id)
      .load::<Partner>(connection)
      .unwrap()
  }

  pub fn update(id: i32, partner: AlreadyPartner, connection: &PgConnection) -> bool {
    let exists = partners::table.find(id).execute(connection);
    match exists {
      Ok(1) => diesel::update(partners::table.find(id))
        .set(&partner)
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    diesel::delete(partners::table.find(id))
      .execute(connection)
      .is_ok()
  }
}
