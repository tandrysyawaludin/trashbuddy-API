use database::schema::transactions;
use diesel;
use diesel::dsl::count;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;

#[table_name = "transactions"]
#[derive(Serialize, Deserialize, Insertable)]
pub struct NewTransaction {
  pub supplier_id: i32,
  pub partner_id: i32,
  pub id_package_of_supplier: i32,
  pub status: String,
}

#[table_name = "transactions"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Transaction {
  pub id: i32,
  pub supplier_id: i32,
  pub partner_id: i32,
  pub id_package_of_supplier: i32,
  pub status: String,
  pub created_at: Option<SystemTime>,
}

#[table_name = "transactions"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct AlreadyTransaction {
  pub supplier_id: i32,
  pub partner_id: i32,
  pub id_package_of_supplier: i32,
  pub status: String,
  pub created_at: Option<SystemTime>,
}

impl Transaction {
  pub fn create(new_transaction: NewTransaction, connection: &PgConnection) -> bool {
    diesel::insert_into(transactions::table)
      .values(&new_transaction)
      .execute(connection)
      .is_ok()
  }

  pub fn read_after_create(connection: &PgConnection) -> Transaction {
    transactions::table
      .order(transactions::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn update(id: i32, transaction: AlreadyTransaction, connection: &PgConnection) -> bool {
    let exists = transactions::table.find(id).limit(1).execute(connection);
    match exists {
      Ok(1) => diesel::update(transactions::table.find(id))
        .set(&transaction)
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    let exists = transactions::table.find(id).limit(1).execute(connection);
    match exists {
      Ok(1) => diesel::delete(transactions::table.find(id))
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn read(page: i64, connection: &PgConnection) -> Vec<Transaction> {
    transactions::table
      .order(transactions::id)
      .limit(10)
      .offset(page * 10)
      .load::<Transaction>(connection)
      .unwrap()
  }

  pub fn count_all(connection: &PgConnection) -> i64 {
    let total = transactions::table
      .select(count(transactions::id))
      .first::<i64>(connection);
    match total {
      Ok(v) => return v,
      Err(_e) => return 0,
    }
  }

  pub fn read_one(id: i32, connection: &PgConnection) -> Vec<Transaction> {
    transactions::table
      .find(id)
      .limit(1)
      .load::<Transaction>(connection)
      .unwrap()
  }
}
