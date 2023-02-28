#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/kayla")]
fn kdawg() -> &'static str {
    "Hello Kayla"
}

#[launch]
fn rocket() -> _ {
    let rock = rocket::build();
    let r2 = rock.mount("/api", routes![index, kdawg]);
    r2.mount("/", routes![index, kdawg])
}