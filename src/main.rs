#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;

mod database {
  pub mod db_setting;
  pub mod schema;
}

mod signin_module;
mod supplier_module;

fn main() {
  rocket::ignite()
    .manage(database::db_setting::connect())
    .mount(
      "/supplier",
      routes![
        supplier_module::create_supplier,
        supplier_module::update_supplier,
        supplier_module::delete_supplier,
        supplier_module::read_one_supplier
      ],
    )
    .mount(
      "/singin_log",
      routes![
        signin_module::create_signin_log,
        signin_module::update_signin_log
      ],
    )
    .mount("/suppliers", routes![supplier_module::read_all_suppliers])
    .mount("/singin_logs", routes![signin_module::read_all_signin_logs])
    .launch();
}
