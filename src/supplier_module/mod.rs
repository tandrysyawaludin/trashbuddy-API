use database;
use rocket_contrib::{Json, Value};
mod supplier;
use self::supplier::{Supplier, NewSupplier, AlreadySupplier};

#[post("/", data = "<supplier>", format = "application/json")]
fn create(supplier: Json<NewSupplier>, connection: database::db_setting::Connection) -> Json<Supplier> {

    // Parse the string of data into serde_json::Value.
    let insert = NewSupplier { ..supplier.into_inner() };
    println!("Please call {} at the number", insert.name);
    Json(Supplier::create(insert, &connection))
}

#[get("/")]
fn read(connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!(Supplier::read(&connection)))
}

#[put("/<id>", data = "<supplier>", format = "application/json")]
fn update(id: i32, supplier: Json<AlreadySupplier>, connection: database::db_setting::Connection) -> Json<Value> {
    let update = AlreadySupplier { ..supplier.into_inner() };
    Json(json!({
        "success": Supplier::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!({
        "success": Supplier::delete(id, &connection)
    }))
}
