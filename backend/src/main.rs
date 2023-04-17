#[macro_use] extern crate rocket;
mod db;
mod api;

use api::{index, kdawg, create_user};

#[launch]
fn rocket() -> _ {
    match db::main() {
        Ok(v) => println!("db connected {v:?}"),
        Err(e) => println!("fuck {e:?}")
    }
    rocket::build()
        .mount("/", routes![index, kdawg, create_user])
}
