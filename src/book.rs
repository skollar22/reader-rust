

pub struct Element {

}

// chapter equivalent - each split = 1 section
pub struct Section {
    pub contents: Vec<Element>,
}

// entire book
pub struct Book {
    pub sections: Vec<Section>,
}