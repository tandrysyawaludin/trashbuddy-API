use database;
use rocket_contrib::{Json, Value};
use self::supplier::{AlreadySupplier, NewSupplier, Supplier, AuthSupplier};
mod supplier;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::http::{Cookie, Cookies};

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
      return Json(json_internal!(
        { 
          "success": success_status, 
          "data": Supplier::read_after_create(&connection)
        }
      ))
    }
    _ => {
      let array: [i32; 0] = [];
      return Json(json_internal!(
        {
          "success": success_status,
          "data": array
        }
      ))
    }
  }
}

#[get("/<page>")]
fn read_all_suppliers(page: i64, token: Authorization, connection: database::db_setting::Connection) -> Json<Value> {
  println!("{:?}", token);
  Json(json_internal!(
    {
      "total": Supplier::count_all(&connection),
      "data": Supplier::read(page, &connection)
    }
  ))
}

#[get("/<id>")]
fn read_one_supplier(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
  Json(json_internal!({ "data": Supplier::read_one(id, &connection) }))
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
  Json(json_internal!(
    {
      "success": Supplier::update(id, update, &connection),
      "data": Supplier::read_one(id, &connection)
    }
  ))
}

#[delete("/<id>")]
fn delete_supplier(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
  Json(json_internal!({ "success": Supplier::delete(id, &connection) }))
}

#[post("/auth", data = "<supplier>", format = "application/json")]
fn auth_supplier(
  supplier: Json<AuthSupplier>,
  connection: database::db_setting::Connection,
  mut cookies: Cookies
) -> Json<Value> {
  let auth = AuthSupplier {
    ..supplier.into_inner()
  };
  let email = auth.email.clone();
  let success_status = Supplier::auth(auth, &connection);
  
  match success_status {
    true => {
      let response_for_jwt = Supplier::read_jwt(email, &connection);
      let encode_jwt = Supplier::encode_jwt(response_for_jwt, &connection);
      cookies.add(Cookie::new("auth_token", encode_jwt.clone()));
      let a = cookies.get("auth_token");
      println!("{:?}", a);

      Json(json_internal!(
        {
          "success": success_status,
          "jwt": encode_jwt,
          "decode_jwt": Supplier::decode_jwt("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.W3siZW1haWwiOiIyYWFALmEiLCJpZCI6MX1d.pePih6txMLPJi_jhu4mQH76RqWYZ5_ivcwsPcysBfq0=".to_string(), &connection)[0]
        }
      ))
    }
    _ => {
      Json(json_internal!(
        {
          "success": success_status,
          "jwt": ""
        }
      ))
    }
  }
}

// Get Headers from Client-Request
#[derive(Debug)]
struct Authorization(String);

fn token_is_valid(key: &str) -> bool {
  return true
}

impl<'a, 'r> FromRequest<'a, 'r> for Authorization {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Authorization, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        println!("{:?}", keys);
        if keys.len() != 1 {
          return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];
        
        if !token_is_valid(keys[0]) {
          println!("masuk");
          return Outcome::Forward(());
        }

        return Outcome::Success(Authorization(key.to_string()));
    }
}
