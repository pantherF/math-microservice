use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::schema::calculations;

#[derive(Queryable, Selectable, Serialize, JsonSchema)]
#[diesel(table_name = calculations)]
pub struct Calculation {
    pub id: i32,
    pub operator: String,
    pub first_number: f64,
    pub second_number: f64,
}

#[derive(Insertable, Deserialize, JsonSchema)]
#[diesel(table_name = calculations)]
pub struct NewCalculation {
    pub operator: String,
    pub first_number: f64,
    pub second_number: f64,
}