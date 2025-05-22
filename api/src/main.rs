mod models;
mod errors;
mod routes;

use rocket::{build, launch, routes};
use rocket_okapi::{swagger_ui::*};

#[launch]  
fn rocket() -> _ {  
    build()  
        .mount("/", routes![
            routes::handle_add,
            routes::handle_subtract,
            routes::handle_multiply,
            routes::handle_divide
        ])
        .mount("/docs", make_swagger_ui(&SwaggerUIConfig {
            url: "/openapi.json".to_owned(),
            ..Default::default()
        }))
}