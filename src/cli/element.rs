use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Element {
    pub atomic_number: u8,
    pub symbol: String,
    pub name: String,
    pub atomic_mass: f64,
    pub group: Option<u8>,
    pub period: u8,
    pub category: ElementCategory,
}

#[derive(Debug, Deserialize)]
pub enum ElementCategory {
    AlkaliMetal,
    AlkalineEarthMetal,
    TransitionMetal,
    PostTransitionMetal,
    Metalloid,
    Nonmetal,
    Halogen,
    NobleGas,
    Lanthanide,
    Actinide,
}

#[derive(Debug, Deserialize)]
pub struct ElementFile {
    pub element: Vec<Element>,
}

pub fn run(input: &str) {
    let text = include_str!("../../data/elements.toml");
    let elements: ElementFile = toml::from_str(text).unwrap();

    let element = elements.element.iter().find(|element| {
        element.name.eq_ignore_ascii_case(input) || element.symbol.eq_ignore_ascii_case(input)
    });

    match element {
        Some(element) => {
            println!("Name:           {}", element.name);
            println!("Symbol:         {}", element.symbol);
            println!("Atomic number:  {}", element.atomic_number);
            println!("Atomic mass:    {}", element.atomic_mass);
            println!("Group:          {}", {
                if let Some(group) = element.group {
                    group.to_string()
                } else {
                    "None".to_string()
                }
            });
            println!("Period:         {}", element.period);
            println!("Category:       {:?}", element.category);
            println!(
                "Neutron count:  {}",
                element.atomic_mass - element.atomic_number as f64
            );
            println!("Proton count:   {}", element.atomic_number);
            println!("Electron count: {}", element.atomic_number);
        }

        None => {
            println!("Element not found: {}", input);
        }
    }
}
