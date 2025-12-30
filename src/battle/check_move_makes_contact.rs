use crate::*;

impl Battle {

    // =========================================================================
    // Miscellaneous Methods (ported from battle.ts)
    // =========================================================================

    /// Check if move makes contact
    /// Equivalent to battle.ts checkMoveMakesContact()
    //
    // 	checkMoveMakesContact(move: ActiveMove, attacker: Pokemon, defender: Pokemon, announcePads = false) {
    // 		if (move.flags['contact'] && attacker.hasItem('protectivepads')) {
    // 			if (announcePads) {
    // 				this.add('-activate', defender, this.effect.fullname);
    // 				this.add('-activate', attacker, 'item: Protective Pads');
    // 			}
    // 			return false;
    // 		}
    // 		return !!move.flags['contact'];
    // 	}
    //
    pub fn check_move_makes_contact(&self, move_id: &ID, attacker: (usize, usize)) -> bool {
        // Check if move has contact flag
        if let Some(move_def) = self.dex.moves().get(move_id.as_str()) {
            if !move_def.flags.contains_key("contact") {
                return false;
            }

            // JS: if (move.flags['contact'] && attacker.hasItem('protectivepads'))
            let (side_idx, poke_idx) = attacker;
            if let Some(side) = self.sides.get(side_idx) {
                if let Some(pokemon) = side.pokemon.get(poke_idx) {
                    // Protective Pads prevents contact
                    if pokemon.item.as_str() == "protectivepads" {
                        return false;
                    }
                }
            }

            return true;
        }
        false
    }
}
