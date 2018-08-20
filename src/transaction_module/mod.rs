use database;
use rocket_contrib::{Json, Value};
mod transaction;
use self::transaction::{AlreadyTransaction, NewTransaction, Transaction};

#[post("/", data = "<transaction>", format = "application/json")]
fn create_transaction(
    transaction: Json<NewTransaction>,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    let insert = NewTransaction {
        ..transaction.into_inner()
    };
    let success_status = Transaction::create(insert, &connection);
    match success_status {
        true => {
            return Json(json_internal!(
        { 
          "success": success_status, 
          "data": Transaction::read_after_create(&connection)
        }
      ))
        }
        _ => {
            let array: [i32; 0] = [];
            return Json(json_internal!(
                {
                "success": success_status,
                "data": array
                }
            ))
        }
    }
}

#[get("/<page>")]
fn read_all_transactions(page: i64, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json_internal!(
    {
      "total": Transaction::count_all(&connection),
      "data": Transaction::read(page, &connection)
    }
  ))
}

#[get("/<id>")]
fn read_one_transaction(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json_internal!({ "data": Transaction::read_one(id, &connection) }))
}

#[put("/<id>", data = "<transaction>", format = "application/json")]
fn update_transaction(
    id: i32,
    transaction: Json<AlreadyTransaction>,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    let update = AlreadyTransaction {
        ..transaction.into_inner()
    };
    Json(json_internal!(
    {
      "success": Transaction::update(id, update, &connection),
      "data": Transaction::read_one(id, &connection)
    }
  ))
}

#[delete("/<id>")]
fn delete_transaction(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json_internal!({ "success": Transaction::delete(id, &connection) }))
}
