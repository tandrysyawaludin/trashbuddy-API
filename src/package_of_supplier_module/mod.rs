use database;
use rocket_contrib::{Json, Value};
mod package_of_supplier;
use self::package_of_supplier::{AlreadyPackageOfSupplier, NewPackageOfSupplier, PackageOfSupplier};

#[post("/", data = "<packages_of_supplier>", format = "application/json")]
fn create_package_of_supplier(
    packages_of_supplier: Json<NewPackageOfSupplier>,
    connection: database::db_setting::Connection,
) -> Json<PackageOfSupplier> {
    // Parse the string of data into serde_json::Value.
    let insert = NewPackageOfSupplier {
        ..packages_of_supplier.into_inner()
    };
    Json(PackageOfSupplier::create(insert, &connection))
}

#[get("/")]
fn read_all_packages_of_supplier(connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!(PackageOfSupplier::read(&connection)))
}

#[get("/<id>")]
fn read_one_package_of_supplier(
    id: i32,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    Json(json!(PackageOfSupplier::read_one(id, &connection)))
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
    Json(json!({
        "success": PackageOfSupplier::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete_package_of_supplier(
    id: i32,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    Json(json!({
        "success": PackageOfSupplier::delete(id, &connection)
    }))
}
