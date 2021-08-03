use std::{collections::BTreeMap, path::PathBuf};
use std::fs::read_to_string;
use glob::{glob, GlobError};
use pulldown_cmark::{html, Parser};

/// Contains all of the HTML generated from the markdown in the given glob
pub struct Posts (Box<BTreeMap<PathBuf, String>>);

impl Posts {
    pub fn new(path: &str) -> Result<Self, GlobError> {
        let mut inner = Box::new(BTreeMap::new());
        for entry in glob(path).expect("Failed to read glob pattern") {
            match entry {
                Ok(title) => {
                    let content = read_to_string(&title).expect("Failed to read file");
                    let parser = Parser::new(&content);
                    let mut html_buf = String::with_capacity(content.capacity() * 3 / 2);
                    html::push_html(&mut html_buf, parser);

                    inner.insert(title.strip_prefix("posts/").unwrap().with_extension(""),  html_buf);
                }
                Err(e) => return Err(e.into())
            }
        }
        Ok(Posts(inner))
    }

    pub fn get(&self, k: String) -> Option<&String> {
        self.0.get(&PathBuf::from(k))
    }
}
