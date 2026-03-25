#[macro_use] extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    let context = context!{
        title: "Home"
    };
    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/public", FileServer::from(relative!("./public")))
        .attach(Template::fairing())
}
