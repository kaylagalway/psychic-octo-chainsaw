#[macro_use] extern crate rocket;

mod db;
mod api;

use api::{index, kdawg, create_user};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, kdawg, create_user])
}
