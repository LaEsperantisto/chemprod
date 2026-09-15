use serde::Deserialize;
use multi_index_map::MultiIndexMap;

#[derive(MultiIndexMap, Debug, Deserialize, Clone)]
pub struct Element {
    #[multi_index(hashed_unique)]
    pub atomic_number: u8,
    #[multi_index(hashed_unique)]
    pub symbol: String,
    #[multi_index(hashed_unique)]
    pub name: String,

    pub atomic_mass: f64,

    #[multi_index(hashed_non_unique)]
    pub group: Option<u8>,
    #[multi_index(hashed_non_unique)]
    pub period: u8,
    #[multi_index(hashed_non_unique)]
    pub category: ElementCategory,
}

#[derive(Debug, Deserialize, Clone, Hash, Eq, PartialEq)]
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
    pub elements: Vec<Element>,
}
pub fn run(input: &str) {
    let text = include_str!("../../data/elements.toml");
    let elements: ElementFile = toml::from_str(text).unwrap();

    let mut map = MultiIndexElementMap::default();
    for element in elements.elements {
        map.insert(element);
    }

    // Attempt lookup by symbol first, then fallback to searching by name
    let found = map
        .get_by_symbol(&input.to_ascii_uppercase())
        .or_else(|| {
            // Case-insensitive lookup by name
            map.iter().find(|e| e.1.name.eq_ignore_ascii_case(input)).map(|(_, element)| element)
        });

    match found {
        Some(element) => {
            println!("Name:           {}", element.name);
            println!("Symbol:         {}", element.symbol);
            println!("Atomic number:  {}", element.atomic_number);
            println!("Atomic mass:    {}", element.atomic_mass);
            println!(
                "Group:          {}",
                element
                    .group
                    .map_or_else(|| "None".to_string(), |g| g.to_string())
            );
            println!("Period:         {}", element.period);
            println!("Category:       {:?}", element.category);
            println!(
                "Neutron count:  {}",
                element.atomic_mass as u8 - element.atomic_number
            );
            println!("Proton count:   {}", element.atomic_number);
            println!("Electron count: {}", element.atomic_number);
        }
        None => {
            println!("Element not found: {}", input);
        }
    }
}