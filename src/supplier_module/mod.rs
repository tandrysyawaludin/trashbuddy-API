use database;
use rocket_contrib::{Json, Value};
mod supplier;
use self::supplier::{AlreadySupplier, NewSupplier, Supplier, AuthSupplier};

#[post("/", data = "<supplier>", format = "application/json")]
fn create_supplier(
  supplier: Json<NewSupplier>,
  connection: database::db_setting::Connection,
) -> Json<Value> {
  // Parse the string of data into serde_json::Value.
  let insert = NewSupplier {
    ..supplier.into_inner()
  };
  let success_status = Supplier::create(insert, &connection);
  match success_status {
    true => {
      return Json(json!(
        { 
          "success": success_status, 
          "data": Supplier::read_after_create(&connection)
        }
      ))
    }
    _ => {
      return Json(json!(
        {
          "success": success_status,
          "data": []
        }
      ))
    }
  }
}

#[get("/<page>")]
fn read_all_suppliers(page: i64, connection: database::db_setting::Connection) -> Json<Value> {
  Json(json!(
    {
      "total": Supplier::count_all(&connection),
      "data": Supplier::read(page, &connection)
    }
  ))
}

#[get("/<id>")]
fn read_one_supplier(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
  Json(json!({ "data": Supplier::read_one(id, &connection) }))
}

#[put("/<id>", data = "<supplier>", format = "application/json")]
fn update_supplier(
  id: i32,
  supplier: Json<AlreadySupplier>,
  connection: database::db_setting::Connection,
) -> Json<Value> {
  let update = AlreadySupplier {
    ..supplier.into_inner()
  };
  Json(json!(
    {
      "success": Supplier::update(id, update, &connection),
      "data": Supplier::read_one(id, &connection)
    }
  ))
}

#[delete("/<id>")]
fn delete_supplier(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
  Json(json!({ "success": Supplier::delete(id, &connection) }))
}

#[post("/auth", data = "<supplier>", format = "application/json")]
fn auth_supplier(
  supplier: Json<AuthSupplier>,
  connection: database::db_setting::Connection,
) -> Json<Value> {
  // Parse the string of data into serde_json::Value.
  let auth = AuthSupplier {
    ..supplier.into_inner()
  };
  let success_status = Supplier::auth(auth.email.clone(), auth.password, &connection);
  match success_status {
    true => {
      let response_for_jwt = Supplier::read_jwt(auth.email, &connection);
      Json(json!(
        {
          "success": success_status,
          "jwt": Supplier::decode_jwt("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.W3siZW1haWwiOiIyYWFALmEiLCJpZCI6MX1d.pePih6txMLPJi_jhu4mQH76RqWYZ5_ivcwsPcysBfq0".to_string(), &connection)
        }
      ))
    }
    _ => {
      Json(json!(
        {
          "success": success_status,
          "jwt": ""
        }
      ))
    }
  }
}
