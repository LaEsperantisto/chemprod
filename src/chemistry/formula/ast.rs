use crate::cli::element::MultiIndexElementMap;
use std::fmt::Debug;

pub trait Formula: Debug {
    fn get_mass(&self, map: &MultiIndexElementMap) -> f64;
    fn get_charge(&self, map: &MultiIndexElementMap) -> i32;
}

#[derive(Debug)]
pub struct FormulaGroup {
    pub formulae: Vec<Box<dyn Formula>>,
    pub multiplier: i32,
}

impl Formula for FormulaGroup {
    fn get_mass(&self, map: &MultiIndexElementMap) -> f64 {
        let mut mass = 0.0;
        self.formulae
            .iter()
            .for_each(|formula| mass += formula.get_mass(map));
        mass * self.multiplier as f64
    }

    fn get_charge(&self, map: &MultiIndexElementMap) -> i32 {
        let group_charge: i32 = self
            .formulae
            .iter()
            .map(|formula| formula.get_charge(map))
            .sum();
        group_charge * self.multiplier
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub symbol: String,
    pub multiplier: i32,
}

impl Formula for Symbol {
    fn get_mass(&self, map: &MultiIndexElementMap) -> f64 {
        map.get_by_symbol(&self.symbol).unwrap().atomic_mass * self.multiplier as f64
    }

    fn get_charge(&self, map: &MultiIndexElementMap) -> i32 {
        let element = map.get_by_symbol(&self.symbol).unwrap();
        // Uses default_oxidation_state, fallback oxidation state, or 0
        let base_charge = element
            .default_oxidation_state
            .or_else(|| element.oxidation_states.first().copied())
            .unwrap_or(0);

        (base_charge as i32) * self.multiplier
    }
}