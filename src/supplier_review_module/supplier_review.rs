use database::schema::supplier_reviews;
use diesel;
use diesel::dsl::count;
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
  pub fn create(new_supplier_review: NewSupplierReview, connection: &PgConnection) -> bool {
    diesel::insert_into(supplier_reviews::table)
      .values(&new_supplier_review)
      .execute(connection)
      .is_ok()
  }

  pub fn read_after_create(connection: &PgConnection) -> SupplierReview {
    supplier_reviews::table
      .order(supplier_reviews::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn update(
    id: i32,
    supplier_review: AlreadySupplierReview,
    connection: &PgConnection,
  ) -> bool {
    let exists = supplier_reviews::table
      .find(id)
      .limit(1)
      .execute(connection);
    match exists {
      Ok(1) => diesel::update(supplier_reviews::table.find(id))
        .set(&supplier_review)
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    let exists = supplier_reviews::table
      .find(id)
      .limit(1)
      .execute(connection);
    match exists {
      Ok(1) => diesel::delete(supplier_reviews::table.find(id))
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn read(page: i64, connection: &PgConnection) -> Vec<SupplierReview> {
    supplier_reviews::table
      .order(supplier_reviews::id)
      .limit(10)
      .offset(page * 10)
      .load::<SupplierReview>(connection)
      .unwrap()
  }

  pub fn count_all(connection: &PgConnection) -> i64 {
    let total = supplier_reviews::table
      .select(count(supplier_reviews::id))
      .first::<i64>(connection);
    match total {
      Ok(v) => return v,
      Err(_e) => return 0,
    }
  }

  pub fn read_one(id: i32, connection: &PgConnection) -> Vec<SupplierReview> {
    supplier_reviews::table
      .find(id)
      .limit(1)
      .load::<SupplierReview>(connection)
      .unwrap()
  }
}
