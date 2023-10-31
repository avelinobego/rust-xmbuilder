use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Element {
    name: String,
    value: String,
    elements: Vec<Box<Element>>,
    attribs: Vec<(String, String)>,
}

impl Clone for Element {
    fn clone(&self) -> Self {
        Element {
            name: self.name.clone(),
            value: self.value.clone(),
            elements: self.elements.clone(),
            attribs: self.attribs.clone(),
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[allow(dead_code)]
impl Element {
    pub fn new(name: &'static str) -> Box<Element> {
        Box::new(Element {
            name: String::from(name),
            value: String::default(),
            elements: Vec::default(),
            attribs: Vec::default(),
        })
    }

    pub fn name(self: Element) -> String {
        self.name
    }

    pub fn value(self: &Element) -> String {
        self.value.clone()
    }

    pub fn set_value(self: &mut Element, value: &'static str) {
        self.value = String::from(value);
    }

    pub fn build_xml(self: &mut Element) {
        self.value.clear();

        let mut tag = String::default();
        tag.push_str(self.open_tag().as_str());

        if self.elements.is_empty() {
            tag.push_str(self.value.as_str());
        }

        for e in &mut self.elements {
            e.build_xml();
            tag.push_str(e.value().as_str());
        }
        tag.push_str(self.close_tag().as_str());

        self.value = tag;
    }

    fn open_tag(self: &Element) -> String {
        let mut tag = String::from(self.name.clone());

        for a in &self.attribs {
            tag.push_str(" ");
            tag.push_str(format!("\"{}\"={}", a.0, a.1).as_str());
        }

        format!("<{}>", tag)
    }

    fn close_tag(self: &Element) -> String {
        format!("</{}>", self.name)
    }

    pub fn add_element(self: &mut Element, value: Element) {
        self.elements.push(Box::new(value));
    }

    pub fn elements(self: &Element) -> Vec<Box<Element>> {
        self.elements.clone()
    }

    pub fn add_attrib(self: &mut Element, id: String, value: String) {
        self.attribs.push((id, value));
    }
}
