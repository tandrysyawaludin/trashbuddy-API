use database;
use rocket_contrib::{Json, Value};
mod partner;
use self::partner::{AlreadyPartner, NewPartner, Partner};

#[post("/", data = "<partner>", format = "application/json")]
fn create_partner(
  partner: Json<NewPartner>,
  connection: database::db_setting::Connection,
) -> Json<Partner> {
  // Parse the string of data into serde_json::Value.
  let insert = NewPartner {
    ..partner.into_inner()
  };
  println!("Please call {} at the number", insert.name);
  Json(Partner::create(insert, &connection))
}

#[get("/")]
fn read_all_partners(connection: database::db_setting::Connection) -> Json<Value> {
  Json(json!(Partner::read(&connection)))
}

#[get("/<id>")]
fn read_one_partner(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
  Json(json!(Partner::read_one(id, &connection)))
}

#[put("/<id>", data = "<partner>", format = "application/json")]
fn update_partner(
  id: i32,
  partner: Json<AlreadyPartner>,
  connection: database::db_setting::Connection,
) -> Json<Value> {
  let update = AlreadyPartner {
    ..partner.into_inner()
  };
  Json(json!({
    "success": Partner::update(id, update, &connection)
  }))
}

#[delete("/<id>")]
fn delete_partner(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
  Json(json!({ "success": Partner::delete(id, &connection) }))
}
