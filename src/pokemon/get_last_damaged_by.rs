use crate::*;
use crate::pokemon::Attacker;

impl Pokemon {

    /// Get last damager info
    /// Equivalent to getLastDamagedBy in pokemon.ts
    //
    // 	getLastDamagedBy(filterOutSameSide: boolean) {
    // 		const damagedBy: Attacker[] = this.attackedBy.filter(attacker => (
    // 			typeof attacker.damageValue === 'number' &&
    // 			(filterOutSameSide === undefined || !this.isAlly(attacker.source))
    // 		));
    // 		if (damagedBy.length === 0) return undefined;
    // 		return damagedBy[damagedBy.length - 1];
    // 	}
    //
    /// Refactored to associated function for Battle access (Session 24 Part 55)
    pub fn get_last_damaged_by(
        battle: &Battle,
        pokemon_pos: (usize, usize),
        filter_out_same_side: bool,
    ) -> Option<Attacker> {
        // Phase 1: Extract attacked_by data
        let attacked_by = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return None,
            };
            pokemon.attacked_by.clone()
        };

        // JS: const damagedBy: Attacker[] = this.attackedBy.filter(attacker => (
        // JS:     typeof attacker.damageValue === 'number' &&
        // JS:     (filterOutSameSide === undefined || !this.isAlly(attacker.source))
        // JS: ));

        // Filter attackers by:
        // 1. damage_value is Some (JavaScript: typeof damageValue === 'number')
        // 2. If filterOutSameSide, check !this.isAlly(attacker.source)
        let damaged_by: Vec<Attacker> = attacked_by
            .into_iter()
            .filter(|attacker| {
                // JavaScript: typeof attacker.damageValue === 'number'
                // In Rust: damage_value is Some for numeric damage, None for non-numeric
                if attacker.damage_value.is_none() {
                    return false;
                }

                // If filter_out_same_side is false, include all attackers with numeric damage
                if !filter_out_same_side {
                    return true;
                }

                // Otherwise, filter out allies
                // JS: filterOutSameSide === undefined || !this.isAlly(attacker.source)
                // Check if attacker is NOT an ally (different side)
                !Pokemon::is_ally(battle, pokemon_pos, attacker.source.0)
            })
            .collect();

        // JS: if (damagedBy.length === 0) return undefined;
        if damaged_by.is_empty() {
            return None;
        }

        // JS: return damagedBy[damagedBy.length - 1];
        damaged_by.last().cloned()
    }
}
