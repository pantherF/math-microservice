use rocket::{post, serde::json};
use calc::add;
use crate::{
    models::{CalculationRequest, CalculationResponse},
    errors::ApiError,
};

#[post("/add", format = "json", data = "<request>")]  
pub async fn handle_add(request: json::Json<CalculationRequest>) -> Result<json::Json<CalculationResponse>, ApiError> {  
    let result = add(request.operand1, request.operand2);  
    Ok(json::Json(CalculationResponse { result }))  
}