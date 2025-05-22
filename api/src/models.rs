use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct CalculationRequest {
    pub operand1: f64,
    pub operand2: f64,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct CalculationResponse {
    pub result: f64,
}