use rocket::{post, serde::json::Json, http::Status, State};
use calc::multiply;
use rocket_okapi::openapi;
use crate::models::{CalculationRequest, CalculationResponse};
use db::{save_calculation, Pool};  

#[openapi]
#[post("/multiply", format = "json", data = "<request>")]  
pub async fn handle_multiply(
    pool: &State<Pool>,
    request: Json<CalculationRequest>,
) -> Result<Json<CalculationResponse>, (Status, String)> {
    let result = multiply(request.operand1, request.operand2);
    
    let mut conn = pool.get()  
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;  
      
    save_calculation(&mut conn, "multiply", request.operand1, request.operand2)  
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;  
    
    Ok(Json(CalculationResponse { result }))
}