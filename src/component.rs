use std::collections::HashMap;
use web_sys::{window, Element};

pub struct Component {
    name: String,
    attributes: HashMap<String, String>,
    children: Vec<Box<Renderable>>,
}

impl Component {
    pub fn new(
        name: &str,
        attributes: HashMap<String, String>,
        children: Vec<Box<Renderable>>,
    ) -> Component {
        Component {
            name: name.into(),
            attributes,
            children,
        }
    }
}

fn attr_to_string(key: &str, value: &str) -> String {
    format!(" {}='{}'", key, value)
}

pub trait Renderable {
    #[cfg(target_arch = "wasm32")]
    fn render(&self) -> Element;

    #[cfg(not(target_arch = "wasm32"))]
    fn render(&self) -> String;
}

impl Renderable for Component {
    fn render(&self) -> String {
        let attributes = self
            .attributes
            .iter()
            .map(|(key, value)| attr_to_string(&key, &value))
            .collect::<Vec<String>>()
            .join("");

        let children = self
            .children
            .iter()
            .map(|child| child.render())
            .collect::<Vec<String>>()
            .join("");

        if children.len() > 0 {
            format!("<{}{}>{}<{}>", &self.name, attributes, children, &self.name)
        } else {
            format!("<{}{} />", &self.name, attributes)
        }
    }
}
