#![feature(decl_macro)]

use std::{
    path::{Path, PathBuf},
    fs::read_to_string
};
use rocket::{
    response::Responder,
    http::ContentType,
    fs::NamedFile,
    get,
    routes,
};
use rocket_contrib::templates::Template;
use tera::{Context, Tera};
use serde_json::json;
// use pulldown_cmark::{html, Parser};
use lazy_static::lazy_static;


/// Turns a sinlge content file into a page by placing it within the
/// default template
fn single_serve(path: &str) -> (ContentType, String) {
    let mut context = Context::new();
    context.insert("content", &TEMPLATES.render(path, &Context::new()).unwrap());
    (
        ContentType::HTML,
        TEMPLATES.render(
            "single.html.tera",
            &context
        ).unwrap()
    )
}

#[get("/")]
async fn index() -> (ContentType, String) {
    (ContentType::HTML, TEMPLATES.render("complete.html", &Context::new()).unwrap())
}

#[get("/resume")] 
async fn resume() -> (ContentType, String) {
    (ContentType::HTML, TEMPLATES.render("resume.html", &Context::new()).unwrap())
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    files(PathBuf::from("content/images/river-favicon.png")).await
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(PathBuf::from("content").join(file)).await.ok()
}

lazy_static! { // Allows non-const functions to be called
    static ref TEMPLATES: Tera = match Tera::new("tera/*.html") {
        Ok(mut t) => {
            t.autoescape_on(vec!["html"]);
            t
        },
        Err(e)=> {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
}

#[rocket::main]
async fn main() {
    let _= rocket::build()
        .mount("/", routes![
            index,
            favicon,
            resume,
            files,
        ])
        .launch().await;
}
