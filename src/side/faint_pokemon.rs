use crate::side::*;

impl Side {

    /// Faint a Pokemon
    pub fn faint_pokemon(&mut self, slot: usize) {
        if let Some(Some(idx)) = self.active.get(slot) {
            let idx = *idx;
            if let Some(pokemon) = self.pokemon.get_mut(idx) {
                pokemon.fainted = true;
                pokemon.faint_queued = false;
                pokemon.hp = 0;
            }
            self.fainted_this_turn = Some(idx);
            self.total_fainted += 1;
            self.pokemon_left = self.pokemon_left.saturating_sub(1);
            self.active[slot] = None;
        }
    }
}
