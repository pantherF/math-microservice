use rocket::{
    build, get, launch, routes,
    fs::FileServer,
    response::content::RawHtml,
};
use rocket::fs::relative;
use std::fs::read_to_string;

#[get("/")]
fn index() -> RawHtml<String> {
    match read_to_string("page/index.html") {
        Ok(html) => RawHtml(html),
        Err(e) => {
            println!("Error reading index.html: {:?}", e);
            RawHtml("<h1>Error loading main page</h1>".to_string())
        }
    }
}

#[get("/history")]  
fn history() -> RawHtml<String> {  
    match read_to_string("page/history.html") {  
        Ok(html) => RawHtml(html),  
        Err(e) => {  
            println!("Error reading history.html: {:?}", e);  
            RawHtml("<h1>Error loading history page</h1>".to_string())  
        }  
    }  
}

#[launch]
fn rocket() -> _ {
    build()
        .mount("/", routes![index, history])
        .mount("/page", FileServer::from(relative!("page")))
}
