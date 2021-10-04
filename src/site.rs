//! Defines the Site struct, which traverses the site sources and
//! creates metadata all in one pass

use std::{
    collections::BTreeMap,
    path::PathBuf,
    fs::read_to_string,
};
use handlebars::Handlebars;
use glob::{glob, GlobError};
use pulldown_cmark::{html, Parser};

pub struct Site {
    registry: Box<Handlebars>,
}

impl Site {    
    pub fn new(glob: &str) -> Result<Self, GlobError> {
        let mut registry = Box::new(Handlebars::new());
        for entry in glob(glob).expect("Failed to read glob pattern") {
            let entry = entry?;
            match entry.extension() {
                Some("html") => {
                    
                },
                Some("md") => {},
                Some(_) | None => {}
            }
        }
        Ok(Site{registry})
    }
}