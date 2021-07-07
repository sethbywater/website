use serde;
use std::collections::HashMap;

#[repr(transparent)]
#[derive(serde::Serialize)]
pub struct Content (Box<HashMap<String, String>>);

impl Content {
    pub fn new() -> Self {
        Self ( Box::new(HashMap::<String, String>::new()) )
    }
}
