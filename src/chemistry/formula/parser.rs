use crate::chemistry::formula::ast::{Formula, FormulaGroup, Symbol};

pub struct Parser {
    current: usize,
    text: Vec<char>,
}

impl Parser {
    pub fn new(text: String) -> Self {
        Self {
            current: 0,
            text: text.chars().collect(),
        }
    }

    pub fn parse(&mut self) -> Box<dyn Formula> {
        self.group()
    }

    fn formula(&mut self, peeked_next: char) -> Box<dyn Formula> {
        if peeked_next == '(' {
            self.next();
            self.group()
        } else if peeked_next.is_alphanumeric() {
            self.symbol()
        } else {
            panic!("Unexpected character: {}", peeked_next);
        }
    }

    fn symbol(&mut self) -> Box<dyn Formula> {
        let mut symbol = String::new();

        // Consume the element symbol (e.g., 'O', 'Fe', 'Na')
        while let Some(next) = self.peek_next() {
            if next.is_ascii_alphabetic() {
                // Stop if we hit a new uppercase letter after already having a symbol
                if next.is_ascii_uppercase() && !symbol.is_empty() {
                    break;
                }
                symbol.push(next);
                self.next(); // Consume character
            } else {
                break;
            }
        }

        let mut string_multiplier = String::new();
        // Consume the multiplier if present (e.g., '2')
        while let Some(next) = self.peek_next() {
            if next.is_numeric() {
                string_multiplier.push(next);
                self.next();
            } else {
                break;
            }
        }

        let multiplier = if string_multiplier.is_empty() {
            1
        } else {
            string_multiplier.parse::<i32>().unwrap_or(1)
        };

        Box::new(Symbol { symbol, multiplier })
    }

    fn group(&mut self) -> Box<dyn Formula> {
        let mut formulae = Vec::new();

        while let Some(peeked_next) = self.peek_next() {
            if peeked_next == ')' {
                self.next();
                break;
            }

            formulae.push(self.formula(peeked_next));
        }

        let mut string_multiplier = String::new();
        while let Some(next) = self.peek_next() {
            if next.is_numeric() {
                string_multiplier.push(next);
                self.next();
            } else {
                break;
            }
        }

        let multiplier = if string_multiplier.is_empty() {
            1
        } else {
            string_multiplier.parse::<i32>().unwrap_or(1)
        };

        Box::new(FormulaGroup {
            formulae,
            multiplier,
        })
    }

    fn peek_next(&mut self) -> Option<char> {
        self.text.get(self.current).map(|c| *c)
    }

    fn next(&mut self) -> Option<char> {
        let output = self.peek_next();
        self.current += 1;
        output
    }
}
