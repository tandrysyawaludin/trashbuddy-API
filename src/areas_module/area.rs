use database::schema::provinces;
use database::schema::districts;
use database::schema::sub_districts;
use diesel::*;
use diesel::pg::PgConnection;
use diesel::prelude::RunQueryDsl;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Area {
  pub province_name: String,
  pub district_name: Option<String>,
  pub sub_district_name: Option<String>,
  pub sub_district_id: Option<String>
}

impl Area {
  pub fn read(connection: &PgConnection) -> Vec<Area> {
    provinces::table.left_join(districts::table
    .left_join(sub_districts::table))
    .select((provinces::name, districts::name.nullable(), sub_districts::name.nullable(), sub_districts::id.nullable()))
    .load::<Area>(connection)
    .unwrap()
  }
}
