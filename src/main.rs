#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate frank_jwt;
extern crate regex;
extern crate rocket_cors;
extern crate djangohashers;

#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;

use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};

mod database {
  pub mod db_setting;
  pub mod schema;
}

mod category_of_trash_module;
mod error_handler_module;
mod package_of_supplier_module;
mod partner_module;
mod partner_review_module;
mod report_to_block_module;
mod signin_log_module;
mod supplier_module;
mod supplier_review_module;
mod transaction_module;
mod areas_module;

use rocket::http::Method;
use rocket_cors::{Guard, AllowedOrigins, AllowedHeaders, Responder};

#[get("/")]
fn index() -> io::Result<NamedFile> {
  NamedFile::open("client_app/build/index.html")
}

#[get("/css/<file..>")]
fn css(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("client_app/build/static/css/").join(file)).ok()
}

#[get("/js/<file..>")]
fn js(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("client_app/build/static/js/").join(file)).ok()
}

#[get("/media/<file..>")]
fn media(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("client_app/build/static/media/").join(file)).ok()
}

#[options("/manual")]
fn manual_options(cors: Guard) -> Responder<&str> {
    cors.responder("Manual OPTIONS preflight handling")
}

fn main() {
  let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:3000"]);
  assert!(failed_origins.is_empty());

  let options = rocket_cors::Cors {
      allowed_origins: allowed_origins,
      allowed_methods: vec![Method::Get, Method::Put, Method::Post, Method::Delete]
          .into_iter()
          .map(From::from)
          .collect(),
      allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
      allow_credentials: true,
      ..Default::default()
  };

  rocket::ignite()
    .manage(database::db_setting::connect())
    .mount(
      "/supplier",
      routes![
        supplier_module::create_supplier,
        supplier_module::update_supplier,
        supplier_module::delete_supplier,
        supplier_module::read_one_supplier,
        supplier_module::auth_supplier
      ],
    )
    .mount(
      "/partner",
      routes![
        partner_module::create_partner,
        partner_module::update_partner,
        partner_module::delete_partner,
        partner_module::read_one_partner,
        partner_module::sign_in_partner
      ],
    )
    .mount(
      "/signin_log",
      routes![
        signin_log_module::create_signin_log,
        signin_log_module::update_signin_log
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
      "/transaction",
      routes![
        transaction_module::create_transaction,
        transaction_module::update_transaction,
        transaction_module::delete_transaction,
        transaction_module::read_one_transaction
      ],
    )
    .mount(
      "/report_to_block",
      routes![
        report_to_block_module::create_report_to_block,
        report_to_block_module::update_report_to_block,
        report_to_block_module::delete_report_to_block,
        report_to_block_module::read_one_report_to_block
      ],
    )
    .mount(
      "/supplier_review",
      routes![
        supplier_review_module::create_supplier_review,
        supplier_review_module::update_supplier_review,
        supplier_review_module::delete_supplier_review,
        supplier_review_module::read_one_supplier_review
      ],
    )
    .mount(
      "/partner_review",
      routes![
        partner_review_module::create_partner_review,
        partner_review_module::update_partner_review,
        partner_review_module::delete_partner_review,
        partner_review_module::read_one_partner_review
      ],
    )
    .mount(
      "/supplier_reviews",
      routes![supplier_review_module::read_all_supplier_reviews],
    )
    .mount(
      "/partner_reviews",
      routes![partner_review_module::read_all_partner_reviews],
    )
    .mount(
      "/packages_of_supplier",
      routes![package_of_supplier_module::read_all_packages_of_supplier],
    )
    .mount(
      "/reports_to_block",
      routes![report_to_block_module::read_all_reports_to_block],
    )
    .mount(
      "/categories_of_trash",
      routes![category_of_trash_module::read_all_categories],
    )
    .mount(
      "/transactions",
      routes![transaction_module::read_all_transactions],
    )
    .mount("/areas", routes![areas_module::read_all_areas])
    .mount("/suppliers", routes![supplier_module::read_all_suppliers])
    .mount("/partners", routes![partner_module::read_all_partners])
    .mount("/signin_logs", routes![signin_log_module::read_all_signin_logs])
    .mount("/", routes![index, manual_options])
    .mount("/static", routes![css, js, media])
    .catch(catchers![
      error_handler_module::internal_error,
      error_handler_module::not_found,
      error_handler_module::unmatch_request
    ])
    .attach(options)
    .launch();
}
