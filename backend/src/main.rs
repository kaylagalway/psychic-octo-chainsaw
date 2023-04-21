#[macro_use] extern crate rocket;
mod api;

use self::models::*;
use diesel::{prelude, QueryDsl, RunQueryDsl, ExpressionMethods};
use psychic_octo_chainsaw::*;
use api::{index, kdawg, create_user};

#[launch]
fn rocket() -> _ {
    use self::schema::users::dsl::*;

    let conn = &mut establish_connection();
    let results = users
    .filter(displayname.eq("davis"))
    .load::<User>(conn).expect("jesus christ");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.email);
        println!("-----------\n");
        println!("{}", post.displayname);
    }

    rocket::build()
        .mount("/", routes![index, kdawg, create_user])
}
