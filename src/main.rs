#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket_contrib::{Json, Value};

mod db;
mod schema;

mod supplier;
use supplier::{Supplier, NewSupplier};

mod signin_log;
use signin_log::{User_role};

mod transaction;
use transaction::{Status_transaction};

#[post("/", data = "<supplier>", format = "application/json")]
fn create(supplier: Json<NewSupplier>, connection: db::Connection) -> Json<Supplier> {

    // Parse the string of data into serde_json::Value.
    let insert = NewSupplier { ..supplier.into_inner() };
    println!("Please call {} at the number", insert.name);
    Json(NewSupplier::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<Value> {
    Json(json!(Supplier::read(&connection)))
}

#[put("/<id>", data = "<supplier>", format = "application/json")]
fn update(id: i32, supplier: Json<Supplier>, connection: db::Connection) -> Json<Value> {
    let update = Supplier { id: id, ..supplier.into_inner() };
    Json(json!({
        "success": Supplier::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": Supplier::delete(id, &connection)
    }))
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/supplier", routes![create, update, delete])
        .mount("/suppliers", routes![read])
        .launch();
}
