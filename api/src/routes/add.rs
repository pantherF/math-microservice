use rocket::{post, serde::json::Json, http::Status};
use calc::add;
use rocket_okapi::openapi;
use crate::models::{CalculationRequest, CalculationResponse};

#[openapi]
#[post("/add", format = "json", data = "<request>")]
pub async fn handle_add(
    request: Json<CalculationRequest>,
) -> Result<Json<CalculationResponse>, (Status, String)> {
    let result = add(request.operand1, request.operand2);
    Ok(Json(CalculationResponse { result }))
}