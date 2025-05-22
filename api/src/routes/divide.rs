use rocket::{post, serde::json};
use calc::divide;
use crate::{
    models::{CalculationRequest, CalculationResponse},
    errors::ApiError,
};

#[post("/divide", format = "json", data = "<request>")]  
pub async fn handle_divide(request: json::Json<CalculationRequest>) -> Result<json::Json<CalculationResponse>, ApiError> {  
    let result = match divide(request.operand1, request.operand2) {
        Some(result) => result,
        None => return Err(ApiError::InvalidInput),
    };

    Ok(json::Json(CalculationResponse { result }))  
}