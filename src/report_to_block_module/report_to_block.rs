use database::schema::reports_to_block;
use diesel;
use diesel::dsl::count;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "reports_to_block"]
pub struct NewReportToBlock {
  pub target_user: i32,
  pub comment: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "reports_to_block"]
pub struct ReportToBlock {
  pub id: i32,
  pub target_user: i32,
  pub comment: String,
  pub created_at: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "reports_to_block"]
pub struct AlreadyReportToBlock {
  pub target_user: i32,
  pub comment: String,
  pub created_at: Option<SystemTime>,
}

impl ReportToBlock {
  pub fn create(new_report_to_block: NewReportToBlock, connection: &PgConnection) -> bool {
    diesel::insert_into(reports_to_block::table)
      .values(&new_report_to_block)
      .execute(connection)
      .is_ok()
  }
  pub fn read_after_create(connection: &PgConnection) -> ReportToBlock {
    reports_to_block::table
      .order(reports_to_block::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn update(id: i32, report_to_block: AlreadyReportToBlock, connection: &PgConnection) -> bool {
    let exists = reports_to_block::table
      .find(id)
      .limit(1)
      .execute(connection);
    match exists {
      Ok(1) => diesel::update(reports_to_block::table.find(id))
        .set(&report_to_block)
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    let exists = reports_to_block::table
      .find(id)
      .limit(1)
      .execute(connection);
    match exists {
      Ok(1) => diesel::delete(reports_to_block::table.find(id))
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn read(page: i64, connection: &PgConnection) -> Vec<ReportToBlock> {
    reports_to_block::table
      .order(reports_to_block::id)
      .limit(10)
      .offset(page * 10)
      .load::<ReportToBlock>(connection)
      .unwrap()
  }

  pub fn count_all(connection: &PgConnection) -> i64 {
    let total = reports_to_block::table
      .select(count(reports_to_block::id))
      .first::<i64>(connection);
    match total {
      Ok(v) => return v,
      Err(_e) => return 0,
    }
  }

  pub fn read_one(id: i32, connection: &PgConnection) -> Vec<ReportToBlock> {
    reports_to_block::table
      .find(id)
      .limit(1)
      .load::<ReportToBlock>(connection)
      .unwrap()
  }
}
