#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket_contrib::{Json, Value};

mod db;
mod schema;

mod customer;
use customer::{Customer, NewCustomer};

#[post("/", data = "<customer>", format = "application/json")]
fn create(customer: Json<NewCustomer>, connection: db::Connection) -> Json<Customer> {

    // Parse the string of data into serde_json::Value.
    let insert = NewCustomer { ..customer.into_inner() };
    println!("Please call {} at the number", insert.name);
    Json(NewCustomer::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<Value> {
    Json(json!(Customer::read(&connection)))
}

#[put("/<id>", data = "<customer>", format = "application/json")]
fn update(id: i32, customer: Json<Customer>, connection: db::Connection) -> Json<Value> {
    let update = Customer { id: id, ..customer.into_inner() };
    Json(json!({
        "success": Customer::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": Customer::delete(id, &connection)
    }))
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/customer", routes![create, update, delete])
        .mount("/customers", routes![read])
        .launch();
}
