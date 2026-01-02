use crate::*;

impl Pokemon {

    /// Clear ability (set to empty)
    ///
    /// This is an associated function (not a method) because it calls set_ability.
    /// Call as: Pokemon::clear_ability(battle, pokemon_pos)
    //
    // 	clearAbility() {
    // 		return this.setAbility('');
    // 	}
    //
    pub fn clear_ability(battle: &mut Battle, pokemon_pos: (usize, usize)) -> ID {
        // JS: return this.setAbility('');
        Pokemon::set_ability(battle, pokemon_pos, ID::empty(), None, None, false, false)
    }
}
