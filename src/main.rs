use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

mod schema;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::sales)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Sale {
    pub id: Option<i32>,
    pub shop_id: i32,
    pub amount: i32,
}

#[derive(Insertable)]
#[diesel(table_name = schema::sales)]
pub struct NewSale {
    pub shop_id: i32,
    pub amount: i32,
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut SqliteConnection, shop_id: i32, amount: i32) -> Sale {
    let new_post = NewSale { shop_id, amount };

    diesel::insert_into(schema::sales::table)
        .values(&new_post)
        .returning(Sale::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

fn main() {
    let mut conn = establish_connection();
    let sale = create_post(&mut conn, 1, 1200);
    println!("Saved post {:?}", sale);
}
