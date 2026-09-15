use crate::cli::element::Element;

pub struct Formula {
    pub elements: Vec<FormulaElement>,
    pub charge: i32,
}

pub struct FormulaElement {
    pub element: &'static Element,
    pub count: u32,
}
