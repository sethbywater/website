//! Defines the Site struct, which traverses the site sources and
//! creates metadata all in one pass

use std::{collections::{BTreeMap, HashMap, HashSet}, fs::read_to_string, path::PathBuf};
use glob::{glob, GlobError};
use notify::Error;
use pulldown_cmark::{html, Parser, Event};
use rocket::form::{Form, prelude::context};
use tera::{Tera, Context};
use serde_json::{json, Value};
use crate::date::Date;

/// Holds all the site's pages in a big `BTreeMap`, and also all metadata
pub struct Site {
    pages: Box<BTreeMap<PathBuf, String>>, 
    posts: Vec<Value>,
}

impl Site {
    pub fn new(dir: &str) -> Result<Self, Box<dyn Error>> {
        let tera = Tera::new(&dir)?;
        let mut pages = Vec::new();
        
        // Every Markdown file must be rendered first so that the templates
        // may use their metadata
        let md_glob = format!("{}{}", dir, "**/*.md");
        for entry in glob(&dir).expect(format!("Failed to read glob: {}", md_glob)) {
            let path = entry?;
            let key =
                path.strip_prefix(&dir)
                    .unwrap()
                    .with_extension("");
            let (context, html) = handle_md(path);
            posts.push(context);

            // Find the corresponding md.html file

            pages.insert(key, tera.render(md_template, &context));
        }

        let html_glob = format!("{}{}", dir, "**/*.html");
        for entry in glob(&dir).expect(format!("Failed to read glob: {}", html_glob)) {
            let path = entry?;
            let key = 
                path.strip_prefix(&dir)
                    .unwrap()
                    .with_extension("");
            if path.ends_with("md.html") | path.ends_with("base.html") { 
                continue // Just a template; these should be cli arguments
            }
        }
        Ok( Site { pages, posts } )
    }

    pub fn render(request: &PathBuf) -> String {
        String::new()
    }
}

fn handle_md(path: &str) -> (Value, String) {
    let mut context = Value::new();

    let path = PathBuf::from(path);
    let file = read_to_string(&path).expect("Failed to read file");
    let parser = Parser::new(&content).peekable();
    
    // Metadata must be preceded by a rule on the first line
    if parser.peek() = Event::Rule {
        parser.next(); // Ignore the first rule
        while let Some(event) = parser.next() {
            match event {
                Event::Text(text) => {
                    let data = text.into_string().split(": ");
                    context.insert(data.next()?, data.next()?);
                },
                // Break on anything other than text
                // In practice, this should always be a rule
                _ => break,
            }
        }
    } else {
        title = 
            path.clone()
                .split('/')
                .last()
                .unwrap();
        date = Date::from_formatted("11111111");
    }

    let mut html = String::with_capacity(content.capacity() * 3 / 2);
    html::push_html(&mut html, parser);

    (context, html)
}