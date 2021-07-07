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
// use lazy_static::*;

// Initialize templates
// lazy_static! {
//     static ref TERA: Tera = compile_templates!("layouts");
// }

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::<String, String>::new();
    context.insert("content".to_string(), "Hello There!".to_string());
    Template::render("index", context)
}

#[get("/resume")] 
fn resume() -> Template {
    let mut context = HashMap::<String, String>::new();
    context.insert(
        "content".to_string(),
        read_to_string(Path::new("../templates/partials/resume.html")).expect("Could open resume")
    );
    Template::render("index", context)
}

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("content/images/river-favicon.png")).ok()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(file).ok()
}

#[catch(404)]
fn not_found() -> Option<NamedFile> {
    NamedFile::open(Path::new("src/404.html")).ok()
}

fn main() {
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
