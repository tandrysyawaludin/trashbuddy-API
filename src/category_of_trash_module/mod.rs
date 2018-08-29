use database;
use rocket_contrib::{Json, Value};
mod category_of_trash;
use self::category_of_trash::{AlreadyCategoryOfTrash, CategoryOfTrash, NewCategoryOfTrash};

#[post("/", data = "<categories_of_trash>", format = "application/json")]
fn create_category(
    categories_of_trash: Json<NewCategoryOfTrash>,
    connection: database::db_setting::Connection,
) -> Json<CategoryOfTrash> {
    // Parse the string of data into serde_json::Value.
    let insert = NewCategoryOfTrash {
        ..categories_of_trash.into_inner()
    };
    Json(CategoryOfTrash::create(insert, &connection))
}

#[get("/")]
fn read_all_categories(connection: database::db_setting::Connection) -> Json<Value> {
    Json(json_internal!(CategoryOfTrash::read(&connection)))
}

#[get("/<name>")]
fn read_one_category(name: String, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json_internal!(CategoryOfTrash::read_one(name, &connection)))
}

#[put("/<name>", data = "<categories_of_trash>", format = "application/json")]
fn update_category(
    name: String,
    categories_of_trash: Json<AlreadyCategoryOfTrash>,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    let update = AlreadyCategoryOfTrash {
        ..categories_of_trash.into_inner()
    };
    Json(json_internal!({
        "success": CategoryOfTrash::update(name, update, &connection)
    }))
}

#[delete("/<name>")]
fn delete_category(name: String, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json_internal!({
        "success": CategoryOfTrash::delete(name, &connection)
    }))
}
