use psychic_octo_chainsaw::models::NewUser;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::serde::json::Error;
use psychic_octo_chainsaw::*;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/kayla")]
pub fn kdawg() -> &'static str {
    "Hello Kayla"
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct NewUser {
//     email: String,
//     password: String,
//     username: String
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthData {
    email: String,
    password: String
}

#[post("/create_user", format = "json", data = "<user_data>")]
pub fn create_user(user_data: Result<Json<NewUser>, Error>) -> Json<String> {
    match user_data {
        Ok(json) => {
            let new_user = json.into_inner();
            println!("received msg: {}", new_user.email);
            let user = NewUser {
                email: new_user.email,
                passhash: new_user.passhash,
                display_name: new_user.display_name
            };
            insert_record(user);
            Json(String::from("Successful user creation"))
        }   
        Err(_) => Json(String::new())
    }
}

#[post("/login", format = "json", data = "<auth_data>")]
pub fn auth_user(auth_data: Result<Json<AuthData>, Error>) -> Json<String> {
    match auth_data {
        Ok(json) => {
            println!("received authdata: {}", json.email);
            Json(String::from("Successful login"))
        }
        Err(_) => Json(String::new())
    }
}
