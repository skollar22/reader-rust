use std::{fmt::Display, fs::File, vec};

use zip::{read::ZipFile, ZipArchive};

use crate::constr::xml::XMLDiv;

pub trait Element {
    fn load(&self);
}

/// href, id
#[derive(Clone)]
pub enum BookItem {
    XHTML(String, String),
    JPEG(String, String),
    CSS(String, String),
    UNKNOWN(String, String),
    NAI
}

// chapter equivalent - each split = 1 section
pub struct Section {
    item: BookItem,
    pub contents: Vec<Box<dyn Element>>,
}

// entire book
pub struct Book {
    pub filepath: String,
    pub sections: Vec<Section>,
    pub manifest: Vec<BookItem>,
}

impl BookItem {
    pub fn get_href(&self) -> &str {
        match self {
            Self::XHTML(href, _) => href,
            Self::JPEG(href, _) => href,
            Self::CSS(href, _) => href,
            Self::UNKNOWN(href, _) => href,
            _ => &""
        }
    }

    pub fn get_id(&self) -> &str {
        match self {
            Self::XHTML(_, id) => id,
            Self::JPEG(_, id) => id,
            Self::CSS(_, id) => id,
            Self::UNKNOWN(_, id) => id,
            _ => &""
        }
    }
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
    pub fn new(filepath: &str) -> Self {
        Book { filepath: filepath.to_string(), sections: vec![], manifest: vec![] }
    }

    pub fn get_zip(&self) -> ZipArchive<File> {
        let file = std::fs::File::open(self.filepath.clone()).expect("Could not find file!");
        ZipArchive::new(file).expect("Error unzipping file!")
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

impl Section {
    pub fn new(file: &BookItem) -> Self {
        Section { item: file.to_owned(), contents: vec![] }
    }

    pub fn load(&self, zip: &mut ZipArchive<File>) {
        let f= zip.by_name(self.item.get_href()).expect("Resource not found in the given archive!");

    }
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.nice())
    }
}