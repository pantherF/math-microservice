mod models;
mod routes;

use rocket::{build, launch};
use rocket_okapi::{openapi_get_routes, swagger_ui::*};
use db::establish_connection_pool;

use crate::routes::add::{handle_add, okapi_add_operation_for_handle_add_};
use crate::routes::subtract::{handle_subtract, okapi_add_operation_for_handle_subtract_};
use crate::routes::multiply::{handle_multiply, okapi_add_operation_for_handle_multiply_};
use crate::routes::divide::{handle_divide, okapi_add_operation_for_handle_divide_};
use crate::routes::calculations::{get_all_calculations, okapi_add_operation_for_get_all_calculations_};

#[launch]  
fn rocket() -> _ {
    dotenvy::dotenv().ok();
    
    let pool = establish_connection_pool();

    build()
        .manage(pool)
        .mount("/", openapi_get_routes![
            handle_add,  
            handle_subtract,  
            handle_multiply,  
            handle_divide,
            get_all_calculations
        ])
        .mount("/docs", make_swagger_ui(&SwaggerUIConfig {
            url: "/openapi.json".to_owned(),
            ..Default::default()
        }))
}