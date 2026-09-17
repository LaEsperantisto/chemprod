use std::collections::HashMap;

use crate::{
    chemistry::formula::parser::Parser,
    cli::element::{ElementFile, MultiIndexElementMap},
};

pub fn run(formula: &str) {
    let mut parser = Parser::new(formula.to_string());
    let tree = parser.parse();

    let text = include_str!("../../data/elements.toml");
    let elements: ElementFile = toml::from_str(text).unwrap();

    let mut map = MultiIndexElementMap::default();
    for element in elements.element {
        map.insert(element);
    }

    let mut elements = HashMap::new();

    println!("\nTotal relative mass: {}", tree.get_mass(&map));
    println!("Overall charge: {}", tree.get_charge(&map));

    println!("\nElement counts:");
    tree.get_elements(&mut elements);
    for (element, count) in elements.iter() {
        println!("{}: {}{}", element, " ".repeat(2 - element.len()), count);
    }
}
