use crate::*;

impl<'a> BattleActions<'a> {

    // =========================================================================
    // MEGA EVOLUTION METHODS
    // =========================================================================

    /// Check if Pokemon can Mega Evolve
    /// Equivalent to canMegaEvo in battle-actions.ts
    //
    // 	// #endregion
    //
    // 	// #region MEGA EVOLUTION
    // 	// ==================================================================
    //
    // 	canMegaEvo(pokemon: Pokemon) {
    // 		const species = pokemon.baseSpecies;
    // 		const altForme = species.otherFormes && this.dex.species.get(species.otherFormes[0]);
    // 		const item = pokemon.getItem();
    // 		// Mega Rayquaza
    // 		if ((this.battle.gen <= 7 || this.battle.ruleTable.has('+pokemontag:past') ||
    // 			this.battle.ruleTable.has('+pokemontag:future')) &&
    // 			altForme?.isMega && altForme?.requiredMove &&
    // 			pokemon.baseMoves.includes(toID(altForme.requiredMove)) && !item.zMove) {
    // 			return altForme.name;
    // 		}
    // 		// Temporary hardcode until generation shift
    // 		if ((species.baseSpecies === "Floette" || species.baseSpecies === "Zygarde") && item.megaEvolves === species.name) {
    // 			return item.megaStone as string;
    // 		}
    // 		// a hacked-in Megazard X can mega evolve into Megazard Y, but not into Megazard X
    // 		if (Array.isArray(item.megaStone)) {
    // 			// FIXME: Change to species.name when champions comes
    // 			const index = (item.megaEvolves as string[]).indexOf(species.baseSpecies);
    // 			if (index < 0) return null;
    // 			return item.megaStone[index];
    // 			// FIXME: Change to species.name when champions comes
    // 		} else if (item.megaEvolves === species.baseSpecies && item.megaStone !== species.name) {
    // 			return item.megaStone;
    // 		}
    // 		return null;
    // 	}
    //
    pub fn can_mega_evo(
        species_name: &str,
        species_other_formes: Option<&[String]>,
        item_mega_evolves: Option<&str>,
        item_mega_stone: Option<&str>,
        base_moves: &[ID],
        item_is_z_move: bool,
        _gen: u8,
    ) -> Option<String> {
        // Check Mega Rayquaza (requires Dragon Ascent)
        if let Some(other_formes) = species_other_formes {
            if let Some(first_forme) = other_formes.first() {
                if first_forme.ends_with("-Mega") {
                    // Check if it requires a move (like Rayquaza)
                    let required_move = ID::new("dragonascent");
                    if base_moves.contains(&required_move) && !item_is_z_move {
                        return Some(first_forme.clone());
                    }
                }
            }
        }

        // Check item-based mega evolution
        if let (Some(mega_evolves), Some(mega_stone)) = (item_mega_evolves, item_mega_stone) {
            // Check if item's mega evolves matches species
            if mega_evolves == species_name && mega_stone != species_name {
                return Some(mega_stone.to_string());
            }
        }

        None
    }
}
