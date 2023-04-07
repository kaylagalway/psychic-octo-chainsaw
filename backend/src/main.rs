#[macro_use] extern crate rocket;
use rocket::Response;
use rocket::http::{ContentType, Status};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::Error;


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
fn create_user(user_data: Result<Json<User>, Error>) -> Result<Response<'static>, Status> {
    match user_data {
        Ok(json) => {
            let message = json.into_inner();
            println!("received msg: {}", user_data.message);
            Ok(Response::build())
                .status(Status::Ok)
                .header(ContentType::Plain)
                .body("got it")
        }   
        Err(_) => Err(Status::BadRequest)
    }
}

#[launch]
fn rocket() -> _ {
    let rock = rocket::build();
    let r2 = rock.mount("/api", routes![index, kdawg]);
    r2.mount("/", routes![index, kdawg])
}
