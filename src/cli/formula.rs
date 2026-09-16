use crate::{chemistry::formula::parser::Parser, cli::element::{Element, ElementFile, MultiIndexElementMap}};

pub struct Formula {
    pub elements: Vec<FormulaElement>,
    pub charge: i32,
}

pub struct FormulaElement {
    pub element: &'static Element,
    pub count: u32,
}

pub fn run(formula: &str) {
    let mut parser = Parser::new();
    let tree = parser.parse();

    let text = include_str!("../../data/elements.toml");
    let elements: ElementFile = toml::from_str(text).unwrap();

    let mut map = MultiIndexElementMap::default();
    for element in elements.element {
        map.insert(element);
    }

    println!("{}",tree.get_mass(&map));
}
