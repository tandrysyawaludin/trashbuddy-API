use database;
use rocket_contrib::{Json, Value};
mod supplier_review;
use self::supplier_review::{AlreadySupplierReview, NewSupplierReview, SupplierReview};

#[post("/", data = "<supplier_reviews>", format = "application/json")]
fn create_supplier_review(
    supplier_reviews: Json<NewSupplierReview>,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    // Parse the string of data into serde_json::Value.
    let insert = NewSupplierReview {
        ..supplier_reviews.into_inner()
    };
    let success_status = SupplierReview::create(insert, &connection);
    match success_status {
        true => {
            return Json(json!(
        { 
          "success": success_status, 
          "data": SupplierReview::read_after_create(&connection)
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
fn read_all_supplier_reviews(
    page: i64,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    Json(json!(
    {
      "total": SupplierReview::count_all(&connection),
      "data": SupplierReview::read(page, &connection)
    }
  ))
}

#[get("/<id>")]
fn read_one_supplier_review(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!({ "data": SupplierReview::read_one(id, &connection) }))
}

#[put("/<id>", data = "<supplier_reviews>", format = "application/json")]
fn update_supplier_review(
    id: i32,
    supplier_reviews: Json<AlreadySupplierReview>,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    let update = AlreadySupplierReview {
        ..supplier_reviews.into_inner()
    };
    Json(json!({
        "success": SupplierReview::update(id, update, &connection),
        "data": SupplierReview::read_one(id, &connection)        
    }))
}

#[delete("/<id>")]
fn delete_supplier_review(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!({
        "success": SupplierReview::delete(id, &connection)
    }))
}
