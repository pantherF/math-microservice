use rocket::{post, serde::json};
use calc::subtract;
use crate::{
    models::{CalculationRequest, CalculationResponse},
    errors::ApiError,
};

#[post("/subtract", format = "json", data = "<request>")]  
pub async fn handle_subtract(request: json::Json<CalculationRequest>) -> Result<json::Json<CalculationResponse>, ApiError> {  
    let result = subtract(request.operand1, request.operand2);  
    Ok(json::Json(CalculationResponse { result }))  
}