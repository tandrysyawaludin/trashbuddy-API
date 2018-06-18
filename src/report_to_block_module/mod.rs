use database;
use rocket_contrib::{Json, Value};
mod report_to_block;
use self::report_to_block::{AlreadyReportToBlock, NewReportToBlock, ReportToBlock};

#[post("/", data = "<report_to_block>", format = "application/json")]
fn create_report_to_block(
    report_to_block: Json<NewReportToBlock>,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    // Parse the string of data into serde_json::Value.
    let insert = NewReportToBlock {
        ..report_to_block.into_inner()
    };
    let success_status = ReportToBlock::create(insert, &connection);
    match success_status {
        true => {
            return Json(json!(
        { 
          "success": success_status, 
          "data": ReportToBlock::read_after_create(&connection)
        }
      ))
        }
        _ => {
            return Json(json!(
        {
          "success": success_status,
          "data": []
        }
      ))
        }
    }
}

#[get("/<page>")]
fn read_all_reports_to_block(
    page: i64,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    Json(json!(
    {
      "total": ReportToBlock::count_all(&connection),
      "data": ReportToBlock::read(page, &connection)
    }
  ))
}

#[get("/<id>")]
fn read_one_report_to_block(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!({ "data": ReportToBlock::read_one(id, &connection) }))
}

#[put("/<id>", data = "<reports_to_block>", format = "application/json")]
fn update_report_to_block(
    id: i32,
    reports_to_block: Json<AlreadyReportToBlock>,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    let update = AlreadyReportToBlock {
        ..reports_to_block.into_inner()
    };
    Json(json!(
    {
      "success": ReportToBlock::update(id, update, &connection),
      "data": ReportToBlock::read_one(id, &connection)
    }
  ))
}

#[delete("/<id>")]
fn delete_report_to_block(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!({ "success": ReportToBlock::delete(id, &connection) }))
}
