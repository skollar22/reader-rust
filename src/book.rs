use std::{fmt::Display, vec};

use crate::constr::xml::XMLDiv;

pub trait Element {
    fn load(&self);
}

pub enum BookItem {
    XHTML(String, String),
    JPEG(String, String),
    CSS(String, String),
    UNKNOWN(String, String),
    NAI
}

// chapter equivalent - each split = 1 section
pub struct Section {
    pub contents: Vec<Box<dyn Element>>,
}

// entire book
pub struct Book {
    pub sections: Vec<Section>,
    pub manifest: Vec<BookItem>,
}

impl From<&XMLDiv> for BookItem {
    fn from(value: &XMLDiv) -> Self {
        if value.get_name() != String::from("item") {
            return Self::NAI;
        }

        let href = value.get_attr("href").unwrap_or("".to_string());
        let id = value.get_attr("id").unwrap_or("".to_string());

        match value.get_attr("media-type").unwrap_or("".to_string()).as_str() {
            "application/xhtml+xml" => Self::XHTML(href, id),
            "image/jpeg" => Self::JPEG(href, id),
            "text/css" => Self::CSS(href, id),
            "" => Self::NAI,
            _ => Self::UNKNOWN(href, id)
        }
    }
}

impl Book {
    pub fn new() -> Self {
        Book { sections: vec![], manifest: vec![] }
    }

    fn nice(&self) -> String {
        let mut output = "manifest: [\n".to_string();
        for i in &self.manifest {
            match i {
                BookItem::XHTML(href, id) => output.push_str(&("\tHTML: ".to_string() + href + " " + id + "\n")),
                BookItem::JPEG(href, id) => output.push_str(&("\tJPEG: ".to_string() + href + " " + id + "\n")),
                BookItem::CSS(href, id) => output.push_str(&("\tCSS : ".to_string() + href + " " + id + "\n")),
                BookItem::UNKNOWN(href, id) => output.push_str(&("\tUNKN: ".to_string() + href + " " + id + "\n")),
                BookItem::NAI => output.push_str(&("\tNon-item\n".to_string()))
            }
        }

        output + "]"
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.nice())
    }
}