use database::schema::transactions;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction {
  pub supplier_id: i32,
  pub partner_id: i32,
  pub id_package_of_supplier: i32,
  pub status: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "transactions"]
pub struct Transaction {
  pub id: i32,
  pub supplier_id: i32,
  pub partner_id: i32,
  pub id_package_of_supplier: i32,
  pub status: String,
  pub created_at: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "transactions"]
pub struct AlreadyTransaction {
  pub supplier_id: i32,
  pub partner_id: i32,
  pub id_package_of_supplier: i32,
  pub status: String,
  pub created_at: Option<SystemTime>,
}

impl Transaction {
  pub fn create(new_transactions: NewTransaction, connection: &PgConnection) -> Transaction {
    diesel::insert_into(transactions::table)
      .values(&new_transactions)
      .execute(connection)
      .expect("Error creating new transactions");

    transactions::table
      .order(transactions::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn read(connection: &PgConnection) -> Vec<Transaction> {
    transactions::table
      .order(transactions::id)
      .load::<Transaction>(connection)
      .unwrap()
  }

  pub fn read_one(id: i32, connection: &PgConnection) -> Vec<Transaction> {
    transactions::table
      .find(id)
      .load::<Transaction>(connection)
      .unwrap()
  }

  pub fn update(id: i32, transactions: AlreadyTransaction, connection: &PgConnection) -> bool {
    diesel::update(transactions::table.find(id))
      .set(&transactions)
      .execute(connection)
      .is_ok()
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    diesel::delete(transactions::table.find(id))
      .execute(connection)
      .is_ok()
  }
}
