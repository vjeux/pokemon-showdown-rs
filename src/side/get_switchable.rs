use crate::side::*;
use crate::*;

impl Side {

    /// Get Pokemon that can switch in
    pub fn get_switchable(&self) -> Vec<usize> {
        self.pokemon
            .iter()
            .enumerate()
            .filter(|(_, p)| !p.is_fainted() && !p.is_active)
            .map(|(i, _)| i)
            .collect()
    }
}
