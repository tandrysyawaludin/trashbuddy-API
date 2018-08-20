use database;
use rocket_contrib::{Json, Value};
mod partner_review;
use self::partner_review::{AlreadyPartnerReview, NewPartnerReview, PartnerReview};

#[post("/", data = "<partner_reviews>", format = "application/json")]
fn create_partner_review(
    partner_reviews: Json<NewPartnerReview>,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    // Parse the string of data into serde_json::Value.
    let insert = NewPartnerReview {
        ..partner_reviews.into_inner()
    };
    let success_status = PartnerReview::create(insert, &connection);
    match success_status {
        true => {
            return Json(json_internal!(
        { 
          "success": success_status, 
          "data": PartnerReview::read_after_create(&connection)
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
fn read_all_partner_reviews(
    page: i64,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    Json(json_internal!(
    {
      "total": PartnerReview::count_all(&connection),
      "data": PartnerReview::read(page, &connection)
    }
  ))
}

#[get("/<id>")]
fn read_one_partner_review(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json_internal!({ "data": PartnerReview::read_one(id, &connection) }))
}

#[put("/<id>", data = "<partner_reviews>", format = "application/json")]
fn update_partner_review(
    id: i32,
    partner_reviews: Json<AlreadyPartnerReview>,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    let update = AlreadyPartnerReview {
        ..partner_reviews.into_inner()
    };
    Json(json_internal!({
        "success": PartnerReview::update(id, update, &connection),
        "data": PartnerReview::read_one(id, &connection)        
    }))
}

#[delete("/<id>")]
fn delete_partner_review(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json_internal!({ "success": PartnerReview::delete(id, &connection) }))
}
