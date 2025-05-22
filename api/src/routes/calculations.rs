use rocket::{get, serde::json::Json, http::Status, State};  
use rocket_okapi::openapi;  

use db::{get_calculations, Pool};

#[openapi]  
#[get("/calculations")]  
pub async fn get_all_calculations(  
    pool: &State<Pool>,  
) -> Result<Json<Vec<db::Calculation>>, (Status, String)> {  
    let mut conn = pool.get()  
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;  
      
    let calculations = get_calculations(&mut conn)  
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;  
      
    Ok(Json(calculations))  
}