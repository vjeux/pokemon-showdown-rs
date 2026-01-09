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
    // TODO: Verify move parameter type matches JavaScript's ActiveMove usage
    pub fn check_move_makes_contact(
        &mut self,
        move_id: &ID,
        attacker: (usize, usize),
        defender: (usize, usize),
        announce_pads: bool,
    ) -> bool {
        // Check if move has contact flag
        if let Some(move_def) = self.dex.moves().get(move_id.as_str()) {
            if !move_def.flags.contains_key("contact") {
                return false;
            }

            // JS: if (move.flags['contact'] && attacker.hasItem('protectivepads'))
            let (attacker_side_idx, attacker_poke_idx) = attacker;
            if let Some(attacker_side) = self.sides.get(attacker_side_idx) {
                if let Some(attacker_pokemon) = attacker_side.pokemon.get(attacker_poke_idx) {
                    // Protective Pads prevents contact
                    if attacker_pokemon.item.as_str() == "protectivepads" {
                        // JS: if (announcePads)
                        if announce_pads {
                            // Get effect name for first message
                            let effect_name = self.current_effect_id()
                                .map(|e| e.to_string())
                                .unwrap_or_default();

                            // Get identifiers for logging
                            let defender_ident = {
                                let (def_side_idx, def_poke_idx) = defender;
                                if let Some(def_side) = self.sides.get(def_side_idx) {
                                    if let Some(def_pokemon) = def_side.pokemon.get(def_poke_idx) {
                                        def_pokemon.get_slot()
                                    } else {
                                        String::from("")
                                    }
                                } else {
                                    String::from("")
                                }
                            };

                            let attacker_ident = attacker_pokemon.get_slot();

                            // JS: this.add('-activate', defender, this.effect.fullname);
                            self.add("-activate", &[defender_ident.as_str().into(), effect_name.as_str().into()]);

                            // JS: this.add('-activate', attacker, 'item: Protective Pads');
                            self.add("-activate", &[attacker_ident.as_str().into(), "item: Protective Pads".into()]);
                        }
                        return false;
                    }
                }
            }

            // JS: return !!move.flags['contact'];
            return true;
        }
        false
    }
}
