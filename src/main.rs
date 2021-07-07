#![feature(decl_macro)]

// mod content;

// use crate::content::Content;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    fs::read_to_string
};
use rocket::{
    response::NamedFile,
    get,
    routes,
    catch,
};
use rocket_contrib::templates::Template;
use tera::{Context, Tera};
use serde_json::json;
use pulldown_cmark::{html, Parser};
use lazy_static::lazy_static;


/// Turns a sinlge content file into a page by placing it within the
/// default template
fn single_serve(path: &str) -> Template {
    let content = read_to_string(Path::new(path)).expect("Found target content");
    Template::render(
        "single",
        json!({
            "content": content,
        })
    )
}

#[get("/")]
fn index() -> String {
    // single_serve("templates/partials/resume.html")
    let mut content = Context::new();
    content.insert("content", "hiiiii");
    TEMPLATES.render(
        "index.html.tera",
        &content
    ).unwrap()
}

#[get("/resume")] 
fn resume() -> String {
    let mut context = Context::new();
    context.insert(
        "content",
        &TEMPLATES.render("partials/resume.html.tera", &Context::new()).expect("Could open resume")
    );
    TEMPLATES.render("index.html.tera", &context).unwrap()
}

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("content/images/river-favicon.png")).ok()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(PathBuf::from("content").join(file)).ok()
}

#[catch(404)]
fn not_found() -> Option<NamedFile> {
    NamedFile::open(Path::new("src/404.html")).ok()
}

lazy_static! { // Allows non-const functions to be called
    static ref TEMPLATES: Tera = match Tera::new("templates/**/*.html.tera") {
        Ok(mut t) => {
            t.autoescape_on(vec!["html", "html.tera"]);
            t
        },
        Err(e)=> {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
}

fn main() {
    println!("pwd: {}", std::env::current_dir().unwrap().display());
    let resume: String = read_to_string("templates/partials/resume.html").expect("Could open resume");
    rocket::ignite()
        .mount("/", routes![
            index,
            favicon,
            resume,
            files
        ])
        .attach(Template::fairing())
        .launch();
}
