use rocket::{post, serde::json::Json, http::Status, State};
use calc::divide;
use rocket_okapi::openapi;
use crate::models::{CalculationRequest, CalculationResponse};
use db::{save_calculation, Pool};

#[openapi]
#[post("/divide", format = "json", data = "<request>")]
pub async fn handle_divide(
    pool: &State<Pool>,
    request: Json<CalculationRequest>,
) -> Result<Json<CalculationResponse>, (Status, String)> {
    match divide(request.operand1, request.operand2) {
        Some(result) => {
            let mut conn = pool.get()
                .map_err(|e| (Status::InternalServerError, e.to_string()))?;
            
            save_calculation(&mut conn, "divide", request.operand1, request.operand2)
                .map_err(|e| (Status::InternalServerError, e.to_string()))?;
            
            Ok(Json(CalculationResponse { result }))
        },
        None => {
            Err((Status::BadRequest, "Cannot divide by zero".to_string()))
        }
    }
}