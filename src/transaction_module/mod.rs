use database;
use rocket_contrib::{Json, Value};
mod transaction;
use self::transaction::{AlreadyTransaction, NewTransaction, Transaction};

#[post("/", data = "<transactions>", format = "application/json")]
fn create_transaction(
    transactions: Json<NewTransaction>,
    connection: database::db_setting::Connection,
) -> Json<Transaction> {
    // Parse the string of data into serde_json::Value.
    let insert = NewTransaction {
        ..transactions.into_inner()
    };
    Json(Transaction::create(insert, &connection))
}

#[get("/")]
fn read_all_transactions(connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!(Transaction::read(&connection)))
}

#[get("/<id>")]
fn read_one_transaction(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!(Transaction::read_one(id, &connection)))
}

#[put("/<id>", data = "<transactions>", format = "application/json")]
fn update_transaction(
    id: i32,
    transactions: Json<AlreadyTransaction>,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    let update = AlreadyTransaction {
        ..transactions.into_inner()
    };
    Json(json!({
        "success": Transaction::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete_transaction(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!({ "success": Transaction::delete(id, &connection) }))
}
