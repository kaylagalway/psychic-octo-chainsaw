#[macro_use] extern crate rocket;
mod api;

use self::models::*;
use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};
use psychic_octo_chainsaw::*;
use api::*;

#[launch]
fn rocket() -> _ {
    use self::schema::users::dsl::*;

    let conn = &mut establish_connection();
    let results = users
    .filter(display_name.eq("davis"))
    .load::<User>(conn).expect("jesus christ");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.email);
        println!("-----------\n");
        println!("{}", post.display_name);
    }

    rocket::build()
        .mount("/api", routes![index, kdawg, create_user, auth_user])
}
