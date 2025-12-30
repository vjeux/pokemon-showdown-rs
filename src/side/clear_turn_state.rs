use crate::side::*;

impl Side {

    /// Clear turn state
    pub fn clear_turn_state(&mut self) {
        self.fainted_last_turn = self.fainted_this_turn;
        self.fainted_this_turn = None;
        self.choice.clear();

        for pokemon in &mut self.pokemon {
            pokemon.clear_turn_state();
        }
    }
}
