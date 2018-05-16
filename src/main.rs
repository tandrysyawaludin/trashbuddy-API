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

mod supplier_module;
use supplier_module::static_rocket_route_info_for_create_supplier;
use supplier_module::static_rocket_route_info_for_delete_supplier;
use supplier_module::static_rocket_route_info_for_read_all_suppliers;
use supplier_module::static_rocket_route_info_for_update_supplier;

mod signin_module;
use signin_module::static_rocket_route_info_for_create_signin_log;
use signin_module::static_rocket_route_info_for_read_all_signin_logs;
use signin_module::static_rocket_route_info_for_update_signin_log;

// mod signin_module;

// mod transaction;
// use transaction::{Status_transaction};

fn main() {
  rocket::ignite()
    .manage(database::db_setting::connect())
    .mount(
      "/supplier",
      routes![create_supplier, update_supplier, delete_supplier],
    )
    .mount("/singin_log", routes![create_signin_log, update_signin_log])
    .mount("/suppliers", routes![read_all_suppliers])
    .mount("/singin_logs", routes![read_all_signin_logs])
    .launch();
}
