use rocket::{post, serde::json::Json, http::Status};
use calc::divide;
use rocket_okapi::openapi;
use crate::models::{CalculationRequest, CalculationResponse};

#[openapi]
#[post("/divide", format = "json", data = "<request>")]
pub async fn handle_divide(
    request: Json<CalculationRequest>,
) -> Result<Json<CalculationResponse>, (Status, String)> {
    match divide(request.operand1, request.operand2) {
        Some(result) => Ok(Json(CalculationResponse { result })),
        None => Err((Status::InternalServerError, "Cannot divide by zero".to_string())),
    }
}
