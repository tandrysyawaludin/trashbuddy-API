use database::schema::supplier_reviews;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "supplier_reviews"]
pub struct NewSupplierReview {
  pub score: i32,
  pub comment: String,
  pub transactions_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "supplier_reviews"]
pub struct SupplierReview {
  pub id: i32,
  pub score: i32,
  pub comment: String,
  pub transactions_id: i32,
  pub created_at: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "supplier_reviews"]
pub struct AlreadySupplierReview {
  pub score: i32,
  pub comment: String,
  pub transactions_id: i32,
  pub created_at: Option<SystemTime>,
}

impl SupplierReview {
  pub fn create(
    new_supplier_reviews: NewSupplierReview,
    connection: &PgConnection,
  ) -> SupplierReview {
    diesel::insert_into(supplier_reviews::table)
      .values(&new_supplier_reviews)
      .execute(connection)
      .expect("Error creating new supplier_reviews");

    supplier_reviews::table
      .order(supplier_reviews::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn read(connection: &PgConnection) -> Vec<SupplierReview> {
    supplier_reviews::table
      .order(supplier_reviews::id)
      .load::<SupplierReview>(connection)
      .unwrap()
  }

  pub fn read_one(id: i32, connection: &PgConnection) -> Vec<SupplierReview> {
    supplier_reviews::table
      .find(id)
      .load::<SupplierReview>(connection)
      .unwrap()
  }

  pub fn update(
    id: i32,
    supplier_reviews: AlreadySupplierReview,
    connection: &PgConnection,
  ) -> bool {
    diesel::update(supplier_reviews::table.find(id))
      .set(&supplier_reviews)
      .execute(connection)
      .is_ok()
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    diesel::delete(supplier_reviews::table.find(id))
      .execute(connection)
      .is_ok()
  }
}
