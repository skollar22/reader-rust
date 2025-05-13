

pub fn verify_mimetype(bytes: Vec<u8>) -> bool {
    let content = String::from_utf8(bytes).unwrap_or("".to_string());
    content == String::from("application/epub+zip")
}