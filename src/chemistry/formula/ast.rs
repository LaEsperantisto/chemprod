use crate::cli::element::MultiIndexElementMap;

pub trait Formula {
    fn get_mass(&self, map: &MultiIndexElementMap) -> f64;
}

pub struct FormulaGroup {
    pub formulae: Vec<Box<dyn Formula>>,
    pub multiplier: i32,
}

impl Formula for FormulaGroup {
    fn get_mass(&self, map: &MultiIndexElementMap) -> f64 {
        let mut mass = 0.0;
        self.formulae
            .iter()
            .for_each(|formula| mass += formula.get_mass(map) * self.multiplier as f64);
        mass
    }
}

pub struct Symbol {
    pub symbol: String,
    pub multiplier: i32,
}

impl Formula for Symbol {
    fn get_mass(&self, map: &MultiIndexElementMap) -> f64 {
        map.get_by_symbol(&self.symbol).unwrap().atomic_mass * self.multiplier as f64
    }
}
