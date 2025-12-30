use crate::side::*;

impl Side {

    /// Get the active Pokemon in a slot
    pub fn get_active(&self, slot: usize) -> Option<&Pokemon> {
        self.active
            .get(slot)
            .and_then(|opt| opt.and_then(|idx| self.pokemon.get(idx)))
    }
}
