use crate::*;
use crate::event::EventResult;

impl Battle {

    /// Swap position of a Pokemon to a new position on their side
    /// Equivalent to battle.ts swapPosition()
    //
    // 	swapPosition(pokemon: Pokemon, newPosition: number, attributes?: string) {
    // 		if (newPosition >= pokemon.side.active.length) {
    // 			throw new Error("Invalid swap position");
    // 		}
    // 		const target = pokemon.side.active[newPosition];
    // 		if (newPosition !== 1 && (!target || target.fainted)) return false;
    //
    // 		this.add('swap', pokemon, newPosition, attributes || '');
    //
    // 		const side = pokemon.side;
    // 		side.pokemon[pokemon.position] = target;
    // 		side.pokemon[newPosition] = pokemon;
    // 		side.active[pokemon.position] = side.pokemon[pokemon.position];
    // 		side.active[newPosition] = side.pokemon[newPosition];
    // 		if (target) target.position = pokemon.position;
    // 		pokemon.position = newPosition;
    // 		this.runEvent('Swap', target, pokemon);
    // 		this.runEvent('Swap', pokemon, target);
    // 		return true;
    // 	}
    //
    pub fn swap_position(
        &mut self,
        pokemon: (usize, usize),
        new_position: usize,
        attributes: Option<&str>,
    ) -> bool {
        let (side_idx, poke_idx) = pokemon;

        // JS: if (newPosition >= pokemon.side.active.length)
        if side_idx >= self.sides.len() {
            return false;
        }

        let active_len = self.active_per_half;
        if new_position >= active_len {
            return false; // throw new Error("Invalid swap position");
        }

        // Get pokemon's current position
        let current_pos = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                pokemon.position
            } else {
                return false;
            }
        } else {
            return false;
        };

        // JS: const target = pokemon.side.active[newPosition];
        // JS: if (newPosition !== 1 && (!target || target.fainted)) return false;
        let target_idx = if let Some(side) = self.sides.get(side_idx) {
            if let Some(target_active) = side.active.get(new_position) {
                if new_position != 1 {
                    if let Some(&idx) = target_active.as_ref() {
                        if let Some(target_poke) = side.pokemon.get(idx) {
                            if target_poke.fainted {
                                return false;
                            }
                        }
                        Some(idx)
                    } else {
                        return false;
                    }
                } else {
                    *target_active
                }
            } else {
                return false;
            }
        } else {
            return false;
        };

        // Log the swap
        let pokemon_str = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                format!("{}: {}", side.id_str(), pokemon.name)
            } else {
                return false;
            }
        } else {
            return false;
        };

        self.add(
            "swap",
            &[
                Arg::String(pokemon_str),
                Arg::String(new_position.to_string()),
                Arg::Str(attributes.unwrap_or("")),
            ],
        );

        // Perform the swap
        // JS: side.pokemon[pokemon.position] = target;
        // JS: side.pokemon[newPosition] = pokemon;
        // JS: side.active[pokemon.position] = side.pokemon[pokemon.position];
        // JS: side.active[newPosition] = side.pokemon[newPosition];
        //
        // Note: JavaScript swaps pokemon in the pokemon array because array index = position.
        // Rust uses indices in the active array (borrow-checker adaptation), so we swap
        // the active array and update position fields instead.
        if let Some(side) = self.sides.get_mut(side_idx) {
            // Swap in active array
            side.active.swap(current_pos, new_position);

            // Update positions
            if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                pokemon.position = new_position;
            }
            if let Some(target_idx) = target_idx {
                if let Some(target) = side.pokemon.get_mut(target_idx) {
                    target.position = current_pos;
                }
            }
        }

        // JS: this.runEvent('Swap', target, pokemon);
        // JS: this.runEvent('Swap', pokemon, target);
        // Fire Swap events for both pokemon involved
        if let Some(target_idx) = target_idx {
            // First event: Swap on target (source=pokemon)
            self.run_event(
                "Swap",
                Some(crate::event::EventTarget::Pokemon((side_idx, target_idx))),
                Some((side_idx, poke_idx)),
                None,
                EventResult::Continue,
                false,
                false,
            );
            // Second event: Swap on pokemon (source=target)
            self.run_event(
                "Swap",
                Some(crate::event::EventTarget::Pokemon((side_idx, poke_idx))),
                Some((side_idx, target_idx)),
                None,
                EventResult::Continue,
                false,
                false,
            );
        }

        true
    }
}
