#[macro_use] extern crate rocket;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::serde::json::Error;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/kayla")]
fn kdawg() -> &'static str {
    "Hello Kayla"
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    message: String
}

#[post("/", format = "json", data = "<user_data>")]
fn create_user(user_data: Result<Json<User>, Error>) -> Json<String> {
    match user_data {
        Ok(json) => {
            let user = json.into_inner();
            println!("received msg: {}", user.message);
            return Json(String::new());
        }   
        Err(_) => Json(String::new())
    }
}

#[launch]
fn rocket() -> _ {
    let rock = rocket::build();
    let r2 = rock.mount("/api", routes![index, kdawg, create_user]);
    r2.mount("/", routes![index, kdawg, create_user])
}
