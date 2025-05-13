use readerlib;

fn main() {
    println!("Hello, world!");
    let book = readerlib::constr::construct("/home/sk/Documents/projects/reader-rust/way_of_kings.epub").unwrap();
    println!("{}", book);
}
