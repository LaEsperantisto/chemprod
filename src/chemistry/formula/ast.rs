use crate::cli::element::{MultiIndexElementMap};
use std::{collections::HashMap, fmt::Debug};

pub trait Formula: Debug {
    fn get_mass(&self, map: &MultiIndexElementMap) -> f64;
    fn get_charge(&self, map: &MultiIndexElementMap) -> i32;
    fn get_elements(&self, map: &mut HashMap<String, usize>);
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

    fn get_elements(&self, map: &mut HashMap<String, usize>) {
        self.formulae.iter().for_each(|formula| {
            for _ in 0..self.multiplier {
                formula.get_elements(map)
            }
        });
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub symbol: String,
    pub multiplier: i32,
}

impl Formula for Symbol {
    fn get_mass(&self, map: &MultiIndexElementMap) -> f64 {
        if let Some(element) = map.get_by_symbol(&self.symbol) {
            element.atomic_mass * self.multiplier as f64
        }else {
            println!("Unexpected character: {}",self.symbol);
            0.0
        }
    }

    fn get_charge(&self, map: &MultiIndexElementMap) -> i32 {
        let Some(element) = map.get_by_symbol(&self.symbol) else {
            println!("Unexpected character: {}",self.symbol);
            return 0;
        };
        // Uses default_oxidation_state, fallback oxidation state, or 0
        let base_charge = element
            .default_oxidation_state
            .or_else(|| element.oxidation_states.first().copied())
            .unwrap_or(0);

        (base_charge as i32) * self.multiplier
    }

    fn get_elements(&self, map: &mut HashMap<String, usize>) {
        if map.contains_key(&self.symbol) {
            *map.get_mut(&self.symbol).unwrap() += self.multiplier as usize;
        } else {
            map.insert(self.symbol.clone(), self.multiplier as usize);
        }
    }
}
