use database;
use rocket_contrib::{Json, Value};
mod area;
use self::area::{Area};

#[get("/")]
fn read_all_areas(connection: database::db_setting::Connection) -> Json<Value> {
    Json(json_internal!(Area::read(&connection)))
}
