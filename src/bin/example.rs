extern crate impulse;

use impulse::component::{Component, Renderable};
use std::collections::HashMap;


fn main() {
    let span = Component::new("span", HashMap::new(), Vec::new());
    let p = Component::new("p", HashMap::new(), vec![Box::new(span)]);

    println!("{}", p.render());
}
