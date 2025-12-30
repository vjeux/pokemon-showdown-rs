use crate::*;

impl Pokemon {

    /// Check if Pokemon has any of the given types
    pub fn has_any_type(&self, types: &[&str]) -> bool {
        let pokemon_types = self.get_types(false);
        for t in types {
            if pokemon_types
                .iter()
                .any(|pt| pt.to_lowercase() == t.to_lowercase())
            {
                return true;
            }
        }
        false
    }
}
