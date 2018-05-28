use database::schema::packages_of_supplier;
use diesel;
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
  pub fn create(
    new_packages_of_supplier: NewPackageOfSupplier,
    connection: &PgConnection,
  ) -> PackageOfSupplier {
    diesel::insert_into(packages_of_supplier::table)
      .values(&new_packages_of_supplier)
      .execute(connection)
      .expect("Error creating new packages_of_supplier");

    packages_of_supplier::table
      .order(packages_of_supplier::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn read(connection: &PgConnection) -> Vec<PackageOfSupplier> {
    packages_of_supplier::table
      .order(packages_of_supplier::id)
      .load::<PackageOfSupplier>(connection)
      .unwrap()
  }

  pub fn read_one(id: i32, connection: &PgConnection) -> Vec<PackageOfSupplier> {
    packages_of_supplier::table
      .find(id)
      .load::<PackageOfSupplier>(connection)
      .unwrap()
  }

  pub fn update(
    id: i32,
    packages_of_supplier: AlreadyPackageOfSupplier,
    connection: &PgConnection,
  ) -> bool {
    diesel::update(packages_of_supplier::table.find(id))
      .set(&packages_of_supplier)
      .execute(connection)
      .is_ok()
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    diesel::delete(packages_of_supplier::table.find(id))
      .execute(connection)
      .is_ok()
  }
}
