use database;
use rocket_contrib::{Json, Value};
mod signin_log;
use self::signin_log::{AlreadySigninLog, NewSigninLog, SigninLog};

#[post("/", data = "<signin_log>", format = "application/json")]
fn create_signin_log(
    signin_log: Json<NewSigninLog>,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    let insert = NewSigninLog {
        ..signin_log.into_inner()
    };
    let success_status = SigninLog::create(insert, &connection);
    match success_status {
        true => {
            return Json(json!(
        { 
          "success": success_status, 
          "data": SigninLog::read_after_create(&connection)
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
fn read_all_signin_logs(page: i64, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!(
    {
      "total": SigninLog::count_all(&connection),
      "data": SigninLog::read(page, &connection)
    }
  ))
}

#[get("/<id>")]
fn read_one_singin_log(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!({ "data": SigninLog::read_one(id, &connection) }))
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
    Json(json!(
    {
      "success": SigninLog::update(id, update, &connection),
      "data": SigninLog::read_one(id, &connection)
    }
  ))
}

#[delete("/<id>")]
fn delete_signin_log(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!({ "success": SigninLog::delete(id, &connection) }))
}
