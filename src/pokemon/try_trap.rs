use crate::*;

impl Pokemon {

    /// Try to trap the Pokemon
    //
    // 	tryTrap(isHidden = false) {
    // 		if (!this.runStatusImmunity('trapped')) return false;
    // 		if (this.trapped && isHidden) return true;
    // 		this.trapped = isHidden ? 'hidden' : true;
    // 		return true;
    // 	}
    //
    pub fn try_trap(battle: &mut Battle, pokemon_pos: (usize, usize), is_hidden: bool) -> bool {
        // JS: if (!this.runStatusImmunity('trapped')) return false;
        let can_be_trapped = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };
            pokemon.run_status_immunity(battle, "trapped")
        };

        if !can_be_trapped {
            return false;
        }

        // JS: if (this.trapped && isHidden) return true;
        let already_trapped = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };
            pokemon.trapped
        };

        if already_trapped && is_hidden {
            return true;
        }

        // JS: this.trapped = isHidden ? 'hidden' : true;
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return false,
        };
        pokemon_mut.trapped = true;
        // Note: Rust trapped field is bool, cannot represent 'hidden' state
        // Note: JavaScript uses bool | 'hidden' to distinguish visible vs hidden trap
        // Note: This is a type system limitation - would need enum Trapped { Visible, Hidden }

        // JS: return true;
        true
    }
}
