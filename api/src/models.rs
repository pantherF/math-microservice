use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]  
#[serde(crate = "rocket::serde")]  
pub struct CalculationRequest {  
    pub operand1: f64,  
    pub operand2: f64,  
}  
  
#[derive(Debug, Serialize)]  
#[serde(crate = "rocket::serde")]  
pub struct CalculationResponse {  
    pub result: f64,  
}