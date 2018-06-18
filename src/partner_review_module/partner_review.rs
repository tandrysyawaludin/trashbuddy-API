use database::schema::partner_reviews;
use diesel;
use diesel::dsl::count;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "partner_reviews"]
pub struct NewPartnerReview {
  pub score: i32,
  pub comment: String,
  pub transactions_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "partner_reviews"]
pub struct PartnerReview {
  pub id: i32,
  pub score: i32,
  pub comment: String,
  pub transactions_id: i32,
  pub created_at: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "partner_reviews"]
pub struct AlreadyPartnerReview {
  pub score: i32,
  pub comment: String,
  pub transactions_id: i32,
  pub created_at: Option<SystemTime>,
}

impl PartnerReview {
  pub fn create(new_partner_review: NewPartnerReview, connection: &PgConnection) -> bool {
    diesel::insert_into(partner_reviews::table)
      .values(&new_partner_review)
      .execute(connection)
      .is_ok()
  }

  pub fn read_after_create(connection: &PgConnection) -> PartnerReview {
    partner_reviews::table
      .order(partner_reviews::id.desc())
      .first(connection)
      .unwrap()
  }

  pub fn update(id: i32, partner_review: AlreadyPartnerReview, connection: &PgConnection) -> bool {
    let exists = partner_reviews::table.find(id).limit(1).execute(connection);
    match exists {
      Ok(1) => diesel::update(partner_reviews::table.find(id))
        .set(&partner_review)
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    let exists = partner_reviews::table.find(id).limit(1).execute(connection);
    match exists {
      Ok(1) => diesel::delete(partner_reviews::table.find(id))
        .execute(connection)
        .is_ok(),
      _ => return false,
    }
  }

  pub fn read(page: i64, connection: &PgConnection) -> Vec<PartnerReview> {
    partner_reviews::table
      .order(partner_reviews::id)
      .limit(10)
      .offset(page * 10)
      .load::<PartnerReview>(connection)
      .unwrap()
  }

  pub fn count_all(connection: &PgConnection) -> i64 {
    let total = partner_reviews::table
      .select(count(partner_reviews::id))
      .first::<i64>(connection);
    match total {
      Ok(v) => return v,
      Err(_e) => return 0,
    }
  }

  pub fn read_one(id: i32, connection: &PgConnection) -> Vec<PartnerReview> {
    partner_reviews::table
      .find(id)
      .limit(1)
      .load::<PartnerReview>(connection)
      .unwrap()
  }
}
