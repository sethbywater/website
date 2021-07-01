#![feature(decl_macro)]

mod index;
use crate::index::render_index;

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket::{get, routes, catch};
use maud::Markup;

#[get("/")]
fn index() -> Markup {
    render_index()
}

#[get("/resume")] 
fn resume() -> Option<NamedFile> {
    NamedFile::open(Path::new("/src/Bywater Resume.pdf")).ok()
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
        .mount("/", routes![index, files])
        .launch();
}