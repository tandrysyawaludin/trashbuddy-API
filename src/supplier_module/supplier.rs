use database::schema::suppliers;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;

#[table_name = "suppliers"]
#[derive(Serialize, Deserialize, Insertable)]
pub struct NewSupplier {
  pub name: String,
  pub email: String,
  pub password: String,
  pub phone_number: String,
  pub area: String,
}

#[table_name = "suppliers"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
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
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct AlreadySupplier {
  pub name: String,
  pub email: String,
  pub password: String,
  pub phone_number: String,
  pub area: String,
  pub created_at: Option<SystemTime>,
}

impl Supplier {
  pub fn create(new_supplier: NewSupplier, connection: &PgConnection) -> Supplier {
    diesel::insert_into(suppliers::table)
      .values(&new_supplier)
      .execute(connection)
      .expect("Error creating new supplier");

    suppliers::table
      .order(suppliers::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn read(connection: &PgConnection) -> Vec<Supplier> {
    suppliers::table
      .order(suppliers::id)
      .load::<Supplier>(connection)
      .unwrap()
  }

  pub fn update(id: i32, supplier: AlreadySupplier, connection: &PgConnection) -> bool {
    diesel::update(suppliers::table.find(id))
      .set(&supplier)
      .execute(connection)
      .is_ok()
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    diesel::delete(suppliers::table.find(id))
      .execute(connection)
      .is_ok()
  }
}
