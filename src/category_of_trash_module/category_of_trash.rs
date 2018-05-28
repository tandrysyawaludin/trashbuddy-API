use database::schema::categories_of_trash;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "categories_of_trash"]
pub struct NewCategoryOfTrash {
  pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "categories_of_trash"]
pub struct CategoryOfTrash {
  pub id: i32,
  pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "categories_of_trash"]
pub struct AlreadyCategoryOfTrash {
  pub name: String,
}

impl CategoryOfTrash {
  pub fn create(
    new_categories_of_trash: NewCategoryOfTrash,
    connection: &PgConnection,
  ) -> CategoryOfTrash {
    diesel::insert_into(categories_of_trash::table)
      .values(&new_categories_of_trash)
      .execute(connection)
      .expect("Error creating new categories_of_trash");

    categories_of_trash::table
      .order(categories_of_trash::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn read(connection: &PgConnection) -> Vec<CategoryOfTrash> {
    categories_of_trash::table
      .order(categories_of_trash::id)
      .load::<CategoryOfTrash>(connection)
      .unwrap()
  }

  pub fn read_one(id: i32, connection: &PgConnection) -> Vec<CategoryOfTrash> {
    categories_of_trash::table
      .find(id)
      .load::<CategoryOfTrash>(connection)
      .unwrap()
  }

  pub fn update(
    id: i32,
    categories_of_trash: AlreadyCategoryOfTrash,
    connection: &PgConnection,
  ) -> bool {
    diesel::update(categories_of_trash::table.find(id))
      .set(&categories_of_trash)
      .execute(connection)
      .is_ok()
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    diesel::delete(categories_of_trash::table.find(id))
      .execute(connection)
      .is_ok()
  }
}
