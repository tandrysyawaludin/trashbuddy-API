use database::schema::reports_to_block;
use diesel;
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
  pub fn create(
    new_reports_to_block: NewReportToBlock,
    connection: &PgConnection,
  ) -> ReportToBlock {
    diesel::insert_into(reports_to_block::table)
      .values(&new_reports_to_block)
      .execute(connection)
      .expect("Error creating new reports_to_block");

    reports_to_block::table
      .order(reports_to_block::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn read(connection: &PgConnection) -> Vec<ReportToBlock> {
    reports_to_block::table
      .order(reports_to_block::id)
      .load::<ReportToBlock>(connection)
      .unwrap()
  }

  pub fn read_one(id: i32, connection: &PgConnection) -> Vec<ReportToBlock> {
    reports_to_block::table
      .find(id)
      .load::<ReportToBlock>(connection)
      .unwrap()
  }

  pub fn update(
    id: i32,
    reports_to_block: AlreadyReportToBlock,
    connection: &PgConnection,
  ) -> Vec<ReportToBlock> {
    diesel::update(reports_to_block::table.find(id))
      .set(&reports_to_block)
      .execute(connection)
      .is_ok();

    reports_to_block::table
      .find(id)
      .load::<ReportToBlock>(connection)
      .unwrap()
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    diesel::delete(reports_to_block::table.find(id))
      .execute(connection)
      .is_ok()
  }
}
