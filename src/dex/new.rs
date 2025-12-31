use crate::*;
use std::collections::HashMap;

impl Dex {
    /// Create a new Dex for a specific generation
    pub fn new(gen: u8) -> Self {
        Self {
            species: HashMap::new(),
            moves: HashMap::new(),
            abilities: HashMap::new(),
            items: HashMap::new(),
            conditions: HashMap::new(),
            types: HashMap::new(),
            natures: HashMap::new(),
            rulesets: HashMap::new(),
            aliases: HashMap::new(),
            compound_word_names: Vec::new(),
            formats: Vec::new(),
            gen,
        }
    }
}
