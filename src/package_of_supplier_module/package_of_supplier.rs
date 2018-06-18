use database::schema::packages_of_supplier;
use diesel;
use diesel::dsl::count;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "packages_of_supplier"]
pub struct NewPackageOfSupplier {
  pub weight: i32,
  pub shipping_fee: i32,
  pub price: i32,
  pub category_of_trash_id: i32,
  pub supplier_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "packages_of_supplier"]
pub struct PackageOfSupplier {
  pub id: i32,
  pub weight: i32,
  pub shipping_fee: i32,
  pub price: i32,
  pub category_of_trash_id: i32,
  pub supplier_id: i32,
  pub created_at: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "packages_of_supplier"]
pub struct AlreadyPackageOfSupplier {
  pub weight: i32,
  pub shipping_fee: i32,
  pub price: i32,
  pub category_of_trash_id: i32,
  pub supplier_id: i32,
  pub created_at: Option<SystemTime>,
}

impl PackageOfSupplier {
  pub fn create(new_package_of_supplier: NewPackageOfSupplier, connection: &PgConnection) -> bool {
    diesel::insert_into(packages_of_supplier::table)
      .values(&new_package_of_supplier)
      .execute(connection)
      .is_ok()
  }

  pub fn read_after_create(connection: &PgConnection) -> PackageOfSupplier {
    packages_of_supplier::table
      .order(packages_of_supplier::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn update(
    id: i32,
    package_of_supplier: AlreadyPackageOfSupplier,
    connection: &PgConnection,
  ) -> bool {
    let exists = packages_of_supplier::table
      .find(id)
      .limit(1)
      .execute(connection);
    match exists {
      Ok(1) => diesel::update(packages_of_supplier::table.find(id))
        .set(&package_of_supplier)
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    let exists = packages_of_supplier::table
      .find(id)
      .limit(1)
      .execute(connection);
    match exists {
      Ok(1) => diesel::delete(packages_of_supplier::table.find(id))
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn read(page: i64, connection: &PgConnection) -> Vec<PackageOfSupplier> {
    packages_of_supplier::table
      .order(packages_of_supplier::id)
      .limit(10)
      .offset(page * 10)
      .load::<PackageOfSupplier>(connection)
      .unwrap()
  }

  pub fn count_all(connection: &PgConnection) -> i64 {
    let total = packages_of_supplier::table
      .select(count(packages_of_supplier::id))
      .first::<i64>(connection);
    match total {
      Ok(v) => return v,
      Err(_e) => return 0,
    }
  }

  pub fn read_one(id: i32, connection: &PgConnection) -> Vec<PackageOfSupplier> {
    packages_of_supplier::table
      .find(id)
      .limit(1)
      .load::<PackageOfSupplier>(connection)
      .unwrap()
  }
}
