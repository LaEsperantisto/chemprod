use crate::cli::element::{Element, ElementFile, MultiIndexElementMap};

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
        self.mass_number.saturating_sub(self.protons())
    }

    pub fn nucleons(&self) -> u16 {
        self.mass_number
    }
}

pub fn run(input: &str) {
    let mut parts = input.split('-').map(str::trim);

    let given_element = match parts.next() {
        Some(s) if !s.is_empty() => s,
        _ => {
            eprintln!("Invalid input format. Expected format like 'Carbon-14' or 'C-14'.");
            return;
        }
    };

    let mass: u16 = match parts.next().and_then(|m| m.parse::<u16>().ok()) {
        Some(m) => m,
        None => {
            eprintln!(
                "Could not parse isotope mass number from: '{input}'. Expected e.g. 'Carbon-14'."
            );
            return;
        }
    };

    let text = include_str!("../../data/elements.toml");
    let elements: ElementFile = toml::from_str(text).unwrap();

    let mut map = MultiIndexElementMap::default();
    for el in elements.element {
        map.insert(el);
    }

    // Fast lookup using symbol index first, fallback to linear search on name
    let found = map
        .get_by_symbol(&given_element.to_ascii_uppercase())
        .or_else(|| {
            map.iter()
                .find(|e| e.1.name.eq_ignore_ascii_case(given_element))
                .map(|(_, element)| element)
        });

    match found {
        Some(element) => {
            let isotope = Isotope {
                element,
                mass_number: mass,
            };

            println!("Name:           {}", isotope.element.name);
            println!("Symbol:         {}", isotope.element.symbol);
            println!("Atomic number:  {}", isotope.element.atomic_number);
            println!("Isotope Mass:   {}", isotope.nucleons());
            println!(
                "Group:          {}",
                isotope
                    .element
                    .group
                    .map_or_else(|| "None".to_string(), |g| g.to_string())
            );
            println!("Period:         {}", isotope.element.period);
            println!("Category:       {:?}", isotope.element.category);
            println!("Neutron count:  {}", isotope.neutrons());
            println!("Proton count:   {}", isotope.protons());
            println!("Electron count: {}", isotope.element.atomic_number);
        }
        None => {
            println!("Element not found: {}", given_element);
        }
    }
}
