use crate::chemistry::formula::ast::{Formula, Symbol};

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&mut self) -> Box<dyn Formula> {
        Box::new(Symbol{symbol:"O".to_string(), multiplier: 1})
    }
}
