use crate::*;

impl Pokemon {

    /// Clear item
    /// Refactored to associated function to match set_item (Session 24 Part 53)
    //
    // 	clearItem() {
    // 		return this.setItem('');
    // 	}
    //
    pub fn clear_item(battle: &mut Battle, pokemon_pos: (usize, usize)) -> bool {
        // JS: return this.setItem('');
        Pokemon::set_item(battle, pokemon_pos, ID::empty(), None, None)
    }
}
