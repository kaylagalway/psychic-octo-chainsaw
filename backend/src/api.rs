use psychic_octo_chainsaw::models::{NewUser, Session};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::serde::json::Error;
use psychic_octo_chainsaw::*;
use uuid::Uuid;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/kayla")]
pub fn kdawg() -> &'static str {
    "Hello Kayla"
}

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
            insert_user(user);
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
            let user = fetch_user(&json.email);
            match user {
                Ok(user) => {
                    if json.password == user.passhash {
                        handle_token(user.id)
                    } else {
                        Json(String::from("You fucked up your password"))
                    }
                }
                Err(_) => {
                    Json(String::from("You fucked up your user fetch"))
                }
            }
        }
        Err(_) => Json(String::from("You fucked up in general"))
    }
}

fn handle_token(user_id: i32) -> Json<String> {
    let session = fetch_session(user_id);
    let token = Uuid::new_v4();
    match session {
        Ok(session) => {
            let update = update_session(
                session, 
                &token.to_string()
            );
            match update {
                Ok(entry_count) => {
                    if entry_count > 0 {
                        Json(String::from("Successful login: nice job updating a session!"))
                    } else {
                        Json(String::from("Failed login: no entries updated!"))
                    }
                }
                Err(_) => {
                    Json(String::from("Failed login: failed updating a session!"))
                }
            }
        }
        Err(_) => {
            let session = Session {
                token: token.to_string(),
                exp_date: 12345,
                user_id: user_id
            };
            insert_session(session);
            Json(String::from("Successful login: nice job creating a session!"))
        }
    }
}
