use std::{collections::HashMap, error::Error, fmt, fs::File, io::Read, str::Chars};

use zip::read::ZipFile;

mod reader;

// ********************************************************************************************************
// ****************************************** XMLError ****************************************************
// ********************************************************************************************************

#[derive(Debug)]
pub struct XMLError {
    msg: String,
}

impl XMLError {
    pub fn new(str: &str) -> Self {
        XMLError { msg: str.to_owned() }
    }
}

impl fmt::Display for XMLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "XMLError: {}", self.msg)
    }
}

impl Error for XMLError {}

// ********************************************************************************************************
// ********************************** Trait and struct definitions ****************************************
// ********************************************************************************************************

// pub trait XMLComponent {
//     fn get_div(&self, name: &str) -> XMLDiv;

//     fn get_content(&self) -> Vec<XMLContent>;

//     fn get_all_children(&self) -> Vec<Box<dyn XMLComponent>>;

//     fn add_div(&self, div: XMLDiv) -> Result<(), XMLError>;

//     fn add_content(&self, cont: XMLContent);
// }

pub type AttrName = String;
pub type AttrVal = String;
pub type Attr = (AttrName, AttrVal);

#[derive(Debug)]
pub enum XMLComponent {
    Div(XMLDiv),
    Content(XMLContent),
    Empty
}

#[derive(Debug)]
pub struct XMLDiv {
    name: String,
    children: Vec<XMLComponent>,
    attributes: HashMap<AttrName, AttrVal>
}

#[derive(Debug)]
pub struct XMLContent {
    cont: String,
}

#[derive(Debug)]
pub struct XML {
    header: Option<XMLDiv>,
    body: Option<XMLDiv>,
    other: HashMap<String, XMLDiv>,
}

struct XMLInStream<'a> {
    str: Chars<'a>,
}

// ********************************************************************************************************
// *********************************** XML implementations ************************************************
// ********************************************************************************************************

impl From<&mut ZipFile<'_, File>> for XML {
    fn from(value: &mut ZipFile<'_, File>) -> Self {
        let mut buf = vec![];
        let _ = value.read_to_end(&mut buf);
        let contents = String::from_utf8(buf).unwrap_or_else(|e| panic!("Could not unwrap component {}!\n{e}", value.name()));
        let contents_nice = contents.replace("\n", "").replace("\r", "");
        let mut stream = XMLInStream::from(&contents_nice);

        let mut document = XML::new();

        while !stream.is_done() {
            stream.skip_to('<');
            if let Some(div) = stream.read_div() {
                if div.get_name() == "body" {
                    document.set_body(div);
                } else if div.get_name() == "header" {
                    document.set_head(div);
                } else {
                    document.add_div(div);
                }
            }
        }

        document
    }
}

impl XML {
    fn new() -> Self {
        XML { header: None, body: None, other: HashMap::new() }
    }

    fn set_body(&mut self, body: XMLDiv) {
        self.body = Some(body);
    }

    fn set_head(&mut self, head: XMLDiv) {
        self.header = Some(head);
    }

    fn add_div(&mut self, div: XMLDiv) {
        self.other.insert(div.get_name(), div);
    }

    pub fn get_div(&self, name: &str) -> Option<&XMLDiv> {
        match name {
            "header" => self.header.as_ref(),
            "body" => self.body.as_ref(),
            other => self.other.get(other)
        }
    }
}

fn strip_enclosing(str: &str) -> String {
    let mut chars = str.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}

fn strip_last_slash(str: &str) -> String {
    if str.ends_with('/') {
        let mut chars = str.chars();
        chars.next_back();
        chars.as_str()
    } else {
        str
    }.to_string()
}

impl XMLInStream<'_> {
    fn skip_to(&mut self, ch: char) {
        let mut curr = self.str.next();
        while curr != Some(ch) && curr != None  {
            curr = self.str.next();
        }
    }

    fn read_until(&mut self, ch: char) -> String {
        let mut curr = self.str.next();
        let mut read_bytes = vec![];
        while curr != Some(ch) && curr != None  {
            read_bytes.push(curr.unwrap());
            curr = self.str.next();
        }
        read_bytes.into_iter().collect::<String>()
    }

    fn read_to_end(&mut self) -> String {
        self.str.clone().collect::<String>()
    }

    fn is_done(&self) -> bool {
        self.str.clone().next() == None
    }

    /// Assumes self.skip_to('<') has already been called
    fn read_div(&mut self) -> Option<XMLDiv> {

        let mut headers = self.read_until('>');
        let mut single_div = false;

        // base cases for recursions
        if headers == String::from("") {
            return None;
        }
        if headers.starts_with(&['/', '!']) {
            // this is a div end, something went wrong
            // OR this is something I don't want to deal with rn
            return None;
        }

        // strip special characters
        if headers.starts_with('?') {
            headers = strip_enclosing(&headers);
            single_div = true;
        }

        if headers.ends_with('/') {
            headers = strip_last_slash(&headers);
            single_div = true;
        }
        

        // we now have a valid div header
        let mut header_stream = XMLInStream::from(&headers);

        // read the div name
        let name = header_stream.read_until(' ');
        let attrs = header_stream.read_to_end();
        drop(header_stream);

        // separate attributes
        let mut attrs_split = vec![];
        for a in attrs.split_whitespace() {
            if a.contains("=") {
                let (an, mut av) = a.split_at(a.find('=').unwrap_or(0));
                av = &av[1..];
                attrs_split.push((an, av));
            } else {
                attrs_split.push((a, ""));
            }
        }


        let mut final_div = XMLDiv::new(&name);
        for (an, av) in attrs_split {
            final_div.add_attr((an.to_string(), strip_enclosing(av)));
        }

        // single divs don't have content
        if single_div {
            return Some(final_div);
        }

        // now parse the contents
        let mut done = false;
        while !done {
            // get any inner contents not in a div
            let contents = self.read_until('<');
            let trimmed =  contents.trim();
            if trimmed != "" {
                final_div.add_content(XMLContent::new(trimmed));
            }

            // check what this new div is
            let cur_div = self.read_div();
            if cur_div.is_none() {
                // we have reached the end of our div
                // since all divs should match their endpoints, 
                // this should always match the correct thing
                done = true;
            } else {
                // this is a new div woo
                final_div.add_div(cur_div.unwrap());
            }
        }

        Some(final_div)
    }
}

impl<'a, 'b> From<&'b String> for XMLInStream<'a> 
where 
    'b: 'a
{
    fn from(value: &'b String) -> Self {
        XMLInStream { str: value.chars() }
    }
}


impl XMLDiv {
    fn new(name: &str) -> Self {
        XMLDiv { name: name.to_string(), children: vec![], attributes: HashMap::new() }
    }

    pub fn get_content(&self) -> Vec<&XMLContent> {
        let mut contents = vec![];
        for c in &self.children {
            if let XMLComponent::Content(cont) = c {
                contents.push(cont);
            }
        }
        contents
    }

    pub fn get_all_children(&self) -> &[XMLComponent] {
        &self.children
    }

    pub fn get_attr(&self, name: &str) -> Option<AttrVal> {
        if self.attributes.contains_key(name) {
            Some(self.attributes.get(name).unwrap().to_owned())
        } else {
            None
        }
        
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_div(&self, name: &str) -> Option<&XMLDiv> {
        for c in &self.children {
            if let XMLComponent::Div(child) = c {
                if child.get_name() == name {
                    return Some(child);
                }
            }
        }
        None
    }

    fn add_div(&mut self, div: XMLDiv) {
        self.children.push(XMLComponent::Div(div));
    }

    fn add_content(&mut self, cont: XMLContent) {
        self.children.push(XMLComponent::Content(cont));
    }

    fn add_attr(&mut self, attr: Attr) {
        self.attributes.insert(attr.0, attr.1);
    }
}

impl XMLContent {
    fn new(cont: &str) -> Self {
        XMLContent { cont: cont.to_owned() }
    }
}