#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

mod database {
    pub mod db_setting;
    pub mod schema;
}

mod supplier_module;
// use supplier_module::static_rocket_route_info_for_create;
// use supplier_module::static_rocket_route_info_for_update;
// use supplier_module::static_rocket_route_info_for_delete;
// use supplier_module::static_rocket_route_info_for_read;

mod signin_module {
    pub mod signin_log;
}
// use signin_module::{User_role};
// use signin_module::static_rocket_route_info_for_create;
// use signin_module::static_rocket_route_info_for_update;
// use signin_module::static_rocket_route_info_for_delete;
// use signin_module::static_rocket_route_info_for_read;

// mod signin_module;

// mod transaction;
// use transaction::{Status_transaction};

fn main() {
    rocket::ignite()
        .manage(database::db_setting::connect())
        // .mount("/supplier", routes![create, update, delete])
        // .mount("/suppliers", routes![read])
        .launch();
}
