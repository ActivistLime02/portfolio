#[macro_use] extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};

#[get("/")]
async fn index() -> Template {
    let context = context!{
        title: "Hoofdpagina",
        navbar_active_state: "index"
    };
    Template::render("pages/index", &context)
}

#[get("/aboutme")]
async fn aboutme() -> Template {
    let context = context! {
        title: "Over mij",
        navbar_active_state: "aboutme"
    };
    Template::render("pages/aboutme", &context)
}

#[get("/cv")]
async fn cv() -> Template {
    let context = context! {
        title: "CV",
        navbar_active_state: "cv"
    };
    Template::render("pages/cv", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index,
            aboutme,
            cv
        ])
        .mount("/public", FileServer::from(relative!("public")))
        .attach(Template::fairing())
}
