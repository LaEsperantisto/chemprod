use std::collections::HashMap;

use crate::cli::element::{Element, ElementFile};

pub struct PeriodicTable {
    elements: Vec<Element>,

    by_symbol: HashMap<String, usize>,
    by_name: HashMap<String, usize>,
}

impl PeriodicTable {
    pub fn from_file(text: &str) -> Result<Self, toml::de::Error> {
        let file: ElementFile = toml::from_str(text)?;

        let mut by_symbol = HashMap::new();
        let mut by_name = HashMap::new();

        for (index, element) in file.element.iter().enumerate() {
            by_symbol.insert(element.symbol.to_lowercase(), index);
            by_name.insert(element.name.to_lowercase(), index);
        }

        Ok(Self {
            elements: file.element,
            by_symbol,
            by_name,
        })
    }

    pub fn get_by_symbol(&self, symbol: &str) -> Option<&Element> {
        let index = self.by_symbol.get(&symbol.to_lowercase())?;

        self.elements.get(*index)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Element> {
        let index = self.by_name.get(&name.to_lowercase())?;

        self.elements.get(*index)
    }
}
