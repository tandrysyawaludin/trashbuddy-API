use database;
use rocket_contrib::{Json, Value};
mod signin_log;
use self::signin_log::{User_role};
use self::signin_log::{SigninLog, NewSigninLog, AlreadySigninLog};

#[post("/", data = "<signin_log>", format = "application/json")]
fn create(signin_log: Json<NewSigninLog>, connection: database::db_setting::Connection) -> Json<SigninLog> {

    // Parse the string of data into serde_json::Value.
    let insert = NewSigninLog { ..signin_log.into_inner() };
    println!("Please call {} at the number", insert.name);
    Json(NewSigninLog::create(insert, &connection))
}

#[get("/")]
fn read(connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!(SigninLog::read(&connection)))
}

#[put("/<id>", data = "<signin_log>", format = "application/json")]
fn update(id: i32, signin_log: Json<AlreadySigninLog>, connection: database::db_setting::Connection) -> Json<Value> {
    let update = AlreadySigninLog { ..signin_log.into_inner() };
    Json(json!({
        "success": SigninLog::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!({
        "success": SigninLog::delete(id, &connection)
    }))
}