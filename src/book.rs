

pub trait Element {
    fn load(&self);
}

// chapter equivalent - each split = 1 section
pub struct Section {
    pub contents: Vec<Box<dyn Element>>,
}

// entire book
pub struct Book {
    pub sections: Vec<Section>,
}