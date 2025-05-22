use rocket::{post, serde::json::Json, http::Status};
use calc::subtract;
use rocket_okapi::openapi;
use crate::models::{CalculationRequest, CalculationResponse};

#[openapi]
#[post("/subtract", format = "json", data = "<request>")]  
pub async fn handle_subtract(
    request: Json<CalculationRequest>,
) -> Result<Json<CalculationResponse>, (Status, String)> {
    let result = subtract(request.operand1, request.operand2);
    Ok(Json(CalculationResponse { result }))
}