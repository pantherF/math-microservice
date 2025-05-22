use rocket::{response, Request, http::Status};

#[derive(Debug)]  
pub enum ApiError {  
    InvalidInput,  
    ServerError,  
}  
  
impl<'r, 'o: 'r> rocket::response::Responder<'r, 'o> for ApiError {  
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {  
        match self {  
            ApiError::InvalidInput => {  
                rocket::response::status::Custom(Status::BadRequest, "Invalid input".to_string()).respond_to(req)  
            }  
            ApiError::ServerError => {  
                rocket::response::status::Custom(Status::InternalServerError, "Server error".to_string()).respond_to(req)  
            }  
        }  
    }  
}