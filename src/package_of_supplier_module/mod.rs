use database;
use rocket_contrib::{Json, Value};
mod package_of_supplier;
use self::package_of_supplier::{AlreadyPackageOfSupplier, NewPackageOfSupplier, PackageOfSupplier};

#[post("/", data = "<packages_of_supplier>", format = "application/json")]
fn create_package_of_supplier(
  packages_of_supplier: Json<NewPackageOfSupplier>,
  connection: database::db_setting::Connection,
) -> Json<Value> {
  let insert = NewPackageOfSupplier {
    ..packages_of_supplier.into_inner()
  };
  let success_status = PackageOfSupplier::create(insert, &connection);
  match success_status {
    true => {
      return Json(json_internal!(
          { 
          "success": success_status,
          "data": PackageOfSupplier::read_after_create(&connection)
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
fn read_all_packages_of_supplier(
  page: i64,
  connection: database::db_setting::Connection,
) -> Json<Value> {
  Json(json_internal!(
    {
      "total": PackageOfSupplier::count_all(&connection),
      "data": PackageOfSupplier::read(page, &connection)
    }
  ))
}

#[get("/<id>")]
fn read_one_package_of_supplier(
  id: i32,
  connection: database::db_setting::Connection,
) -> Json<Value> {
  Json(json_internal!({
    "data": PackageOfSupplier::read_one(id, &connection)
  }))
}

#[put("/<id>", data = "<packages_of_supplier>", format = "application/json")]
fn update_package_of_supplier(
  id: i32,
  packages_of_supplier: Json<AlreadyPackageOfSupplier>,
  connection: database::db_setting::Connection,
) -> Json<Value> {
  let update = AlreadyPackageOfSupplier {
    ..packages_of_supplier.into_inner()
  };
  Json(json_internal!({
        "success": PackageOfSupplier::update(id, update, &connection),
        "data": PackageOfSupplier::read_one(id, &connection)    
    }))
}

#[delete("/<id>")]
fn delete_package_of_supplier(
  id: i32,
  connection: database::db_setting::Connection,
) -> Json<Value> {
  Json(json_internal!({
    "success": PackageOfSupplier::delete(id, &connection)
  }))
}