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

mod category_of_trash_module;
mod package_of_supplier_module;
mod partner_module;
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
      "/partner",
      routes![
        partner_module::create_partner,
        partner_module::update_partner,
        partner_module::delete_partner,
        partner_module::read_one_partner
      ],
    )
    .mount(
      "/singin_log",
      routes![
        signin_module::create_signin_log,
        signin_module::update_signin_log
      ],
    )
    .mount(
      "/package_of_supplier",
      routes![
        package_of_supplier_module::create_package_of_supplier,
        package_of_supplier_module::update_package_of_supplier,
        package_of_supplier_module::delete_package_of_supplier,
        package_of_supplier_module::read_one_package_of_supplier
      ],
    )
    .mount(
      "/category_of_trash",
      routes![
        category_of_trash_module::create_category,
        category_of_trash_module::update_category,
        category_of_trash_module::delete_category,
        category_of_trash_module::read_one_category
      ],
    )
    .mount(
      "/packages_of_supplier",
      routes![package_of_supplier_module::read_all_packages_of_supplier],
    )
    .mount(
      "/categories_of_trash",
      routes![category_of_trash_module::read_all_categories],
    )
    .mount("/suppliers", routes![supplier_module::read_all_suppliers])
    .mount("/partners", routes![partner_module::read_all_partners])
    .mount("/singin_logs", routes![signin_module::read_all_signin_logs])
    .launch();
}
