use database::schema::categories_of_trash;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "categories_of_trash"]
pub struct NewCategoryOfTrash {
  pub name: String,
  pub description: String  
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "categories_of_trash"]
pub struct CategoryOfTrash {
  pub name: String,
  pub description: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "categories_of_trash"]
pub struct AlreadyCategoryOfTrash {
  pub name: String,
  pub description: String  
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
      .first(connection)
      .unwrap()
  }

  pub fn read(connection: &PgConnection) -> Vec<CategoryOfTrash> {
    categories_of_trash::table
      .load::<CategoryOfTrash>(connection)
      .unwrap()
  }

  pub fn read_one(name: String, connection: &PgConnection) -> Vec<CategoryOfTrash> {
    categories_of_trash::table
      .filter(categories_of_trash::name.eq(&name))
      .load::<CategoryOfTrash>(connection)
      .unwrap()
  }

  pub fn update(
    name: String,
    categories_of_trash: AlreadyCategoryOfTrash,
    connection: &PgConnection,
  ) -> bool {
    diesel::update(categories_of_trash::table.filter(categories_of_trash::name.eq(&name)))
      .set(&categories_of_trash)
      .execute(connection)
      .is_ok()
  }

  pub fn delete(name: String, connection: &PgConnection) -> bool {
    diesel::delete(categories_of_trash::table.filter(categories_of_trash::name.eq(&name)))
      .execute(connection)
      .is_ok()
  }
}
