use database;
use rocket_contrib::{Json, Value};
mod report_to_block;
use self::report_to_block::{AlreadyReportToBlock, NewReportToBlock, ReportToBlock};

#[post("/", data = "<reports_to_block>", format = "application/json")]
fn create_report_to_block(
    reports_to_block: Json<NewReportToBlock>,
    connection: database::db_setting::Connection,
) -> Json<ReportToBlock> {
    // Parse the string of data into serde_json::Value.
    let insert = NewReportToBlock {
        ..reports_to_block.into_inner()
    };
    Json(ReportToBlock::create(insert, &connection))
}

#[get("/")]
fn read_all_reports_to_block(connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!(ReportToBlock::read(&connection)))
}

#[get("/<id>")]
fn read_one_report_to_block(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!(ReportToBlock::read_one(id, &connection)))
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
    Json(json!({
        "success": ReportToBlock::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete_report_to_block(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!({ "success": ReportToBlock::delete(id, &connection) }))
}
