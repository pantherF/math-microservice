use rocket::{post, serde::json::Json, http::Status, State};
use calc::subtract;
use rocket_okapi::openapi;
use crate::models::{CalculationRequest, CalculationResponse};
use db::{save_calculation, Pool};  

#[openapi]
#[post("/subtract", format = "json", data = "<request>")]  
pub async fn handle_subtract(
    pool: &State<Pool>,
    request: Json<CalculationRequest>,
) -> Result<Json<CalculationResponse>, (Status, String)> {
    let result = subtract(request.operand1, request.operand2);
    
     let mut conn = pool.get()  
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;  
      
    save_calculation(&mut conn, "subtract", request.operand1, request.operand2)  
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    
    Ok(Json(CalculationResponse { result }))
}