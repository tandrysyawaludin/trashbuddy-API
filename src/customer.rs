use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::customers;

#[table_name = "customers"]
#[derive(Serialize, Deserialize, Insertable)]
pub struct NewCustomer {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone_number: i32
}

#[table_name = "customers"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone_number: i32
}

impl NewCustomer {
    pub fn create(new_customer: NewCustomer, connection: &PgConnection) -> Customer {
        diesel::insert_into(customers::table)
            .values(&new_customer)
            .execute(connection)
            .expect("Error creating new customer");

        customers::table.order(customers::id.desc()).first(connection).unwrap()
    }
}

impl Customer {
    // pub fn create(customer: NewCustomer, connection: &PgConnection) -> NewCustomer {
    //     diesel::insert_into(customers::table).values(&customer).get_result(connection).expect("Error saving new post")
    // }

    pub fn read(connection: &PgConnection) -> Vec<Customer> {
        customers::table.order(customers::id).load::<Customer>(connection).unwrap()
    }

    pub fn update(id: i32, customer: Customer, connection: &PgConnection) -> bool {
        diesel::update(customers::table.find(id)).set(&customer).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(customers::table.find(id)).execute(connection).is_ok()
    }
}
