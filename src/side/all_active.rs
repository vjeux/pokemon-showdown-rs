use crate::side::*;

impl Side {

    /// Get all active Pokemon
    pub fn all_active(&self) -> Vec<&Pokemon> {
        self.active
            .iter()
            .filter_map(|opt| opt.and_then(|idx| self.pokemon.get(idx)))
            .collect()
    }
}
