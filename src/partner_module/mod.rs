use database;
use rocket_contrib::{Json, Value};
mod partner;
use self::partner::{AlreadyPartner, NewPartner, Partner, SignInPartner};

#[post("/", data = "<partner>", format = "application/json")]
fn create_partner(
  partner: Json<NewPartner>,
  connection: database::db_setting::Connection,
) -> Json<Value> {
  let insert = NewPartner {
    ..partner.into_inner()
  };
  let success_status = Partner::create(insert, &connection);
  match success_status {
    true => {
      return Json(json_internal!(
        { 
          "success": success_status, 
          "data": Partner::read_after_create(&connection)
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

#[derive(Debug, FromForm)]
struct Params {
  area: String,
  category: String,
  page: i64
}

#[get("/find?<params>")]
fn read_all_partners(params: Option<Params>, connection: database::db_setting::Connection) -> Json<Value> {
  let data = params.unwrap();
  let page = data.page;
  let area = data.area.clone();
  let category = data.category.clone();

  Json(json_internal!(
    {
      "total": Partner::count_all(area.clone(), category.clone(), &connection),
      "data": Partner::read(page, area, category, &connection)
    }
  ))
}

#[get("/<id>")]
fn read_one_partner(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
  Json(json_internal!({ "data": Partner::read_one(id, &connection) }))
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
  Json(json_internal!(
    {
      "success": Partner::update(id, update, &connection),
      "data": Partner::read_one(id, &connection)
    }
  ))
}

#[delete("/<id>")]
fn delete_partner(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
  Json(json_internal!({ "success": Partner::delete(id, &connection) }))
}

#[post("/sign_in", data = "<partner>", format = "application/json")]
fn sign_in_partner(partner: Json<SignInPartner>, connection: database::db_setting::Connection) -> Json<Value> {
  let data = SignInPartner {
    ..partner.into_inner()
  };
  let success_status = Partner::sign_in(data.email, data.password, &connection);
  let array: [i32; 0] = [];
  return Json(json_internal!(
    { 
      "success": success_status, 
      "data": array
    }
  ))
}
