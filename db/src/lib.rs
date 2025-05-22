extern crate diesel;

pub mod schema;
pub mod models;
pub mod db;

pub use db::{establish_connection_pool, save_calculation, get_calculations, Pool};
pub use models::{Calculation, NewCalculation};