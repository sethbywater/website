#![feature(decl_macro)]

mod posts;

use std::{
    path::PathBuf,
    collections::HashMap,
};
use rocket::{
    http::ContentType,
    fs::NamedFile,
    get,
    routes,
};
use tera::{Context, Tera, Function, Value};
use lazy_static::lazy_static;
use serde_json::{to_value, from_value};

use crate::posts::Posts;

#[get("/")]
async fn index() -> (ContentType, String) {
    (ContentType::HTML, TEMPLATES.render("home.html", &Context::new()).unwrap())
}

#[get("/resume")] 
async fn resume() -> (ContentType, String) {
    (ContentType::HTML, TEMPLATES.render("resume.html", &Context::new()).unwrap())
}

#[get("/works")] 
async fn works() -> (ContentType, String) {
    (ContentType::HTML, TEMPLATES.render("works.html", &Context::new()).unwrap())
}

#[get("/blog")] 
async fn blog() -> (ContentType, String) {
    (ContentType::HTML, TEMPLATES.render("blog.html", &Context::new()).unwrap())
}
#[get("/post/<title>")]
async fn post(title: String) -> (ContentType, String) {
    let mut context = Context::new();
    context.insert("title", &title);
    context.insert("content", POSTS.get(title).expect("Could not find post"));
    (ContentType::HTML, TEMPLATES.render("single.html", &context).unwrap())
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    files(PathBuf::from("content/images/river-favicon.png")).await
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(PathBuf::from("content").join(file)).await.ok()
}

fn get_md() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
        match args.get("name") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => to_value(POSTS.get(v).unwrap()).map_err(|_e| tera::Error::template_not_found(".md file not found")),
                Err(e) => Err(e.into()),
            },
            None => Err(tera::Error::call_function("Invalid arguments", "get_md"))
        }
    })
}

lazy_static! { // Allows non-const functions to be called
    static ref TEMPLATES: Tera = match Tera::new("tera/*.html") {
        Ok(mut t) => {
            t.autoescape_on(vec![]);
            t.register_function("get_md", get_md());
            t
        },
        Err(e)=> {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    static ref POSTS: Posts = match Posts::new("posts/*.md") {
        Ok(p) => p,
        Err(e) => {
            println!("Failed to build posts, error(s): {}", e);
            ::std::process::exit(1);
        }
    };
}

#[rocket::main]
async fn main() {
    let _= rocket::build()
        .mount("/", routes![
            index,
            resume,
            works,
            blog,
            post,
            favicon,
            files,
        ])
        .launch().await;
}
