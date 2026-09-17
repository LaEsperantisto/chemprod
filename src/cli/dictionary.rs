pub const GLOSSARY: &[(&str, &str)] = &[
    ("Acid", "A substance that donates hydrogen ions (H+) in solution or accepts an electron pair."),
    ("Activation Energy", "The minimum amount of energy required to initiate a chemical reaction."),
    ("Anion", "A negatively charged ion formed when an atom gains electrons."),
    ("Base", "A substance that accepts hydrogen ions (H+) or donates an electron pair."),
    ("Cation", "A positively charged ion formed when an atom loses electrons."),
    ("Covalent Bond", "A chemical bond formed when two atoms share one or more pairs of electrons."),
    ("Electronegativity", "A measure of the tendency of an atom to attract a bonding pair of electrons."),
    ("Enthalpy", "A thermodynamic quantity equivalent to the total heat content of a system."),
    ("Ionization Energy", "The energy required to remove an electron from a gaseous atom or ion."),
    ("Ionic Bond", "A bond formed by the electrostatic attraction between oppositely charged ions."),
    ("Isotope", "Atoms of the same element that have the same number of protons but different numbers of neutrons."),
    ("Molarity", "A measure of concentration representing the moles of solute per liter of solution."),
    ("Oxidation", "The loss of electrons during a reaction by a molecule, atom, or ion."),
    ("pH", "A logarithmic scale used to specify the acidity or basicity of an aqueous solution."),
    ("Reduction", "The gain of electrons during a reaction by a molecule, atom, or ion."),
    ("Solute", "The substance that is dissolved in a solution."),
    ("Solvent", "The substance in which a solute dissolves to form a solution."),
    ("Stoichiometry", "The calculation of reactants and products in chemical reactions based on conservation of mass."),
];

fn lookup_term<'a>(search_term: &str) -> Option<&'a str> {
    GLOSSARY
        .iter()
        .find(|(term, _)| term.eq_ignore_ascii_case(search_term))
        .map(|(_, def)| *def)
}

// Calculates how many single-character edits (insertions, deletions, substitutions)
// are required to turn string `a` into string `b`.
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let mut distances = vec![vec![0; b_chars.len() + 1]; a_chars.len() + 1];

    for i in 0..=a_chars.len() {
        distances[i][0] = i;
    }
    for j in 0..=b_chars.len() {
        distances[0][j] = j;
    }

    for i in 1..=a_chars.len() {
        for j in 1..=b_chars.len() {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            distances[i][j] = (distances[i - 1][j] + 1)
                .min(distances[i][j - 1] + 1)
                .min(distances[i - 1][j - 1] + cost);
        }
    }

    distances[a_chars.len()][b_chars.len()]
}

fn find_similar_terms<'a>(search_term: &str) -> Vec<&'a str> {
    let query = search_term.to_lowercase();
    
    GLOSSARY
        .iter()
        .map(|(term, _)| *term)
        .filter(|term| {
            let lower_term = term.to_lowercase();
            
            // 1. Check substring match
            let is_substring = lower_term.contains(&query) || query.contains(&lower_term);
            
            // 2. Check edit distance (allow up to 3 character differences depending on term length)
            let distance = levenshtein_distance(&query, &lower_term);
            let max_allowed_distance = match query.len() {
                0..=4 => 1,
                5..=8 => 2,
                _ => 3,
            };

            is_substring || distance <= max_allowed_distance
        })
        .collect()
}

pub fn run(term: &str) {
    if term == "" {
        println!("{:#?}", GLOSSARY);
    }
    else {
        match lookup_term(term) {
            Some(definition) => {
                println!("{}: {}", term, definition);
            }
            None => {
                println!("Term '{}' not found.", term);
                
                let similar = find_similar_terms(term);
                if !similar.is_empty() {
                    println!("Did you mean: {}?", similar.join(", "));
                }
            }
        }
    }
}