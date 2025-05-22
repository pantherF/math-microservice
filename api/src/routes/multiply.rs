use rocket::{post, serde::json::Json, http::Status};
use calc::multiply;
use rocket_okapi::openapi;
use crate::models::{CalculationRequest, CalculationResponse};

#[openapi]
#[post("/multiply", format = "json", data = "<request>")]  
pub async fn handle_multiply(
    request: Json<CalculationRequest>,
) -> Result<Json<CalculationResponse>, (Status, String)> {
    let result = multiply(request.operand1, request.operand2);
    Ok(Json(CalculationResponse { result }))
}