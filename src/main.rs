#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;

mod controllers;


#[derive(Serialize, Deserialize)]
struct TemplateContext {
    name: String,
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/")]
fn index() -> Template {
    let context = TemplateContext { name: String::from("John") };

    Template::render("index", &context)
}

fn main() {

    rocket::ignite()
        .mount("/", routes![index, files])
        .mount("/posts",
               routes![controllers::post_controller::index,
                       controllers::post_controller::hello])
        .launch()
}
