use crate::*;

impl Battle {

    /// Find a valid switch target for pivot moves (U-Turn, Volt Switch, etc.)
    pub fn find_valid_switch_target(&self, side_idx: usize, current_poke_idx: usize) -> Option<usize> {
        // Find the first non-active, non-fainted Pokemon
        for (idx, pokemon) in self.sides[side_idx].pokemon.iter().enumerate() {
            if idx != current_poke_idx && !pokemon.is_active && !pokemon.is_fainted() {
                return Some(idx);
            }
        }
        None
    }
}
