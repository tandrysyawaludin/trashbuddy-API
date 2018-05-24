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
  pub created_at: Option<SystemTime>,
}

impl Partner {
  pub fn create(new_supplier: NewPartner, connection: &PgConnection) -> Partner {
    diesel::insert_into(partners::table)
      .values(&new_supplier)
      .execute(connection)
      .expect("Error creating new supplier");

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

  pub fn update(id: i32, supplier: AlreadyPartner, connection: &PgConnection) -> bool {
    diesel::update(partners::table.find(id))
      .set(&supplier)
      .execute(connection)
      .is_ok()
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    diesel::delete(partners::table.find(id))
      .execute(connection)
      .is_ok()
  }
}
