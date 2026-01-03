use crate::*;

impl Battle {
    /// Set a Pokemon's trapped state
    /// JavaScript equivalent: pokemon.trapped = true | 'hidden' | false
    ///
    /// In JavaScript, trapped can be:
    /// - false/undefined: not trapped
    /// - true: trapped and visible
    /// - 'hidden': trapped but hidden from opponent
    ///
    /// In Rust, we use TrappedState enum:
    /// - TrappedState::None: not trapped
    /// - TrappedState::Visible: trapped and visible (true in JS)
    /// - TrappedState::Hidden: trapped and hidden ('hidden' in JS)
    pub fn set_trapped(&mut self, pokemon_pos: (usize, usize), state: pokemon::TrappedState) {
        if let Some(pokemon) = self.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.trapped = state;
        }
    }
}
