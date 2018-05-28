use database;
use rocket_contrib::{Json, Value};
mod signin_log;
use self::signin_log::{AlreadySigninLog, NewSigninLog, SigninLog};

#[post("/", data = "<signin_log>", format = "application/json")]
fn create_signin_log(
    signin_log: Json<NewSigninLog>,
    connection: database::db_setting::Connection,
) -> Json<SigninLog> {
    // Parse the string of data into serde_json::Value.
    let insert = NewSigninLog {
        ..signin_log.into_inner()
    };
    Json(SigninLog::create(insert, &connection))
}

#[get("/")]
fn read_all_signin_logs(connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!(SigninLog::read(&connection)))
}

#[put("/<id>", data = "<signin_log>", format = "application/json")]
fn update_signin_log(
    id: i32,
    signin_log: Json<AlreadySigninLog>,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    let update = AlreadySigninLog {
        ..signin_log.into_inner()
    };
    Json(json!({
        "success": SigninLog::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete_signin_log(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!({ "success": SigninLog::delete(id, &connection) }))
}
