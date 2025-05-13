use std::io::Read;

use zip::ZipArchive;

use crate::book::Book;

mod mimetype;
mod xml;

pub fn construct(file_path: &str) -> Option<Book> {
    let file = std::fs::File::open(file_path).expect("Could not find file!");
    let mut zip = ZipArchive::new(file).expect("Error unzipping file!");

    let mut rf_path = "".to_string();

    for i in 0..zip.len() {
        let mut f = zip.by_index(i).unwrap();
        println!("{:?}", f.name());

        if f.name() == "mimetype" {
            let mut buf: Vec<u8> = vec![];
            let _ = f.read_to_end(&mut buf);
            if !mimetype::verify_mimetype(buf) {
                return None;
            }

        } else if f.name() == "META-INF/container.xml" {
            let container = xml::XML::from(&mut f);
            
            let container_err_msg = "Poorly constructed container, cannot read book!";
            let rf_div = container.get_div("container")
                        .expect(container_err_msg)
                        .get_div("rootfiles")
                        .expect(container_err_msg)
                        .get_div("rootfile")
                        .expect(container_err_msg);
            rf_path = rf_div.get_attr("full-path").expect(container_err_msg);
        } else if f.name() == rf_path.as_str() {
            // rootfile, yummy
        }
    }

    Some(Book { sections: vec![] })
}