use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::serde::json::Error;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/kayla")]
pub fn kdawg() -> &'static str {
    "Hello Kayla"
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    message: String
}

#[post("/create_user", format = "json", data = "<user_data>")]
pub fn create_user(user_data: Result<Json<User>, Error>) -> Json<String> {
    match user_data {
        Ok(json) => {
            let user = json.into_inner();
            println!("received msg: {}", user.message);
            return Json(String::new());
        }   
        Err(_) => Json(String::new())
    }
}
