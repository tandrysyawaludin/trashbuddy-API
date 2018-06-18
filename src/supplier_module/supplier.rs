use database::schema::suppliers;
use diesel;
use diesel::dsl::count;
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
  pub fn create(new_supplier: NewSupplier, connection: &PgConnection) -> bool {
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
}
