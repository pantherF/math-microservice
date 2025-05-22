use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

use crate::models::{Calculation, NewCalculation};
use crate::schema::calculations;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_connection_pool() -> Pool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub fn save_calculation(
    conn: &mut DbConnection,
    operator: &str,
    first_number: f64,
    second_number: f64,
) -> QueryResult<Calculation> {
    let new_calculation = NewCalculation {
        operator: operator.to_string(),
        first_number: first_number,
        second_number: second_number,
    };
    
    diesel::insert_into(calculations::table)
        .values(&new_calculation)
        .returning(Calculation::as_returning())
        .get_result(conn)
}

pub fn get_calculations(conn: &mut DbConnection) -> QueryResult<Vec<Calculation>> {
    calculations::table.select(Calculation::as_select()).load(conn)
}