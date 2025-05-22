use rocket::{post, serde::json};
use calc::multiply;
use crate::{
    models::{CalculationRequest, CalculationResponse},
    errors::ApiError,
};

#[post("/multiply", format = "json", data = "<request>")]  
pub async fn handle_multiply(request: json::Json<CalculationRequest>) -> Result<json::Json<CalculationResponse>, ApiError> {  
    let result = multiply(request.operand1, request.operand2);  
    Ok(json::Json(CalculationResponse { result }))  
}