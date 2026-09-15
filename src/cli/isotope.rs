use std::str::FromStr;

use crate::cli::element::{self, Element, ElementFile};

#[derive(Debug)]
pub struct Isotope<'a> {
    pub element: &'a Element,
    pub mass_number: u16,
}

impl Isotope<'_> {
    pub fn protons(&self) -> u16 {
        self.element.atomic_number as u16
    }

    pub fn neutrons(&self) -> u16 {
        self.mass_number - self.protons()
    }

    pub fn nucleons(&self) -> u16 {
        self.mass_number
    }
}

pub fn run(input: &str) {
    let split: Vec<_> = input.split("-").collect();
    let given_element = split.get(0).unwrap_or_else(|| {
        eprintln!(
            "Could not split {} into an element and an atomic mass",
            input
        );
        &"H"
    });
    let mass = split.get(1).unwrap_or_else(|| {
        eprintln!(
            "Could not split {} into an element and an atomic mass",
            input
        );
        &"H"
    }).parse::<u8>()
    .unwrap_or_else(|_| {
        eprintln!(
            "Could not split {} into an element and an atomic mass",
            input
        );
        0
    });
    let text = include_str!("../../data/elements.toml");
    let elements: ElementFile = toml::from_str(text).unwrap();

    let element = elements.element.iter().find(|element| {
        element.name.eq_ignore_ascii_case(given_element)
            || element.symbol.eq_ignore_ascii_case(given_element)
    });

    match element {
        Some(element) => {
            println!("Name:          {}", element.name);
            println!("Symbol:        {}", element.symbol);
            println!("Atomic number: {}", element.atomic_number);
            println!("Atomic mass:   {}", mass);
            println!("Group:         {}", {
                if let Some(group) = element.group {
                    group.to_string()
                } else {
                    "None".to_string()
                }
            });
            println!("Period:        {}", element.period);
            println!("Category:      {:?}", element.category);
            println!("Neutron count: {}", mass.saturating_sub(element.atomic_number));
            println!("Proton count:  {}", element.atomic_number);
            println!("Electron count:{}", element.atomic_number);
        }

        None => {
            println!("Element not found: {}", input);
        }
    }
}
