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
    Json(json!(CategoryOfTrash::read(&connection)))
}

#[get("/<id>")]
fn read_one_category(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!(CategoryOfTrash::read_one(id, &connection)))
}

#[put("/<id>", data = "<categories_of_trash>", format = "application/json")]
fn update_category(
    id: i32,
    categories_of_trash: Json<AlreadyCategoryOfTrash>,
    connection: database::db_setting::Connection,
) -> Json<Value> {
    let update = AlreadyCategoryOfTrash {
        ..categories_of_trash.into_inner()
    };
    Json(json!({
        "success": CategoryOfTrash::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete_category(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
    Json(json!({
        "success": CategoryOfTrash::delete(id, &connection)
    }))
}
