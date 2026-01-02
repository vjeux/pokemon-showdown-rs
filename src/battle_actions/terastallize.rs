//! BattleActions::terastallize - Handle Terastallization
//!
//! 1:1 port of terastallize from battle-actions.ts

// JS Source:
// 	terastallize(pokemon: Pokemon) {
// 		if (pokemon.species.baseSpecies === 'Ogerpon' && !['Fire', 'Grass', 'Rock', 'Water'].includes(pokemon.teraType) &&
// 			(!pokemon.illusion || pokemon.illusion.species.baseSpecies === 'Ogerpon')) {
// 			this.battle.hint("If Ogerpon Terastallizes into a type other than Fire, Grass, Rock, or Water, the game softlocks.");
// 			return;
// 		}
//
// 		if (pokemon.illusion && ['Ogerpon', 'Terapagos'].includes(pokemon.illusion.species.baseSpecies)) {
// 			this.battle.singleEvent('End', this.dex.abilities.get('Illusion'), pokemon.abilityState, pokemon);
// 		}
//
// 		const type = pokemon.teraType;
// 		this.battle.add('-terastallize', pokemon, type);
// 		pokemon.terastallized = type;
// 		for (const ally of pokemon.side.pokemon) {
// 			ally.canTerastallize = null;
// 		}
// 		pokemon.addedType = '';
// 		pokemon.knownType = true;
// 		pokemon.apparentType = type;
// 		if (pokemon.species.baseSpecies === 'Ogerpon') {
// 			let ogerponSpecies = toID(pokemon.species.battleOnly || pokemon.species.id);
// 			ogerponSpecies += ogerponSpecies === 'ogerpon' ? 'tealtera' : 'tera';
// 			pokemon.formeChange(ogerponSpecies, null, true);
// 		}
// 		if (pokemon.species.name === 'Terapagos-Terastal') {
// 			pokemon.formeChange('Terapagos-Stellar', null, true);
// 		}
// 		if (pokemon.species.baseSpecies === 'Morpeko' && !pokemon.transformed &&
// 			pokemon.baseSpecies.id !== pokemon.species.id
// 		) {
// 			pokemon.formeRegression = true;
// 			pokemon.baseSpecies = pokemon.species;
// 			pokemon.details = pokemon.getUpdatedDetails();
// 		}
// 		this.battle.runEvent('AfterTerastallization', pokemon);
// 	}

use crate::*;

/// Handle Terastallization
/// Equivalent to terastallize() in battle-actions.ts
///
/// JavaScript signature:
/// terastallize(pokemon: Pokemon)
pub fn terastallize(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) {
    let (side_index, pokemon_index) = pokemon_pos;

    // Phase 1: Extract data immutably
    let (base_species, tera_type, _has_illusion) = {
        let pokemon = match battle.pokemon_at(side_index, pokemon_index) {
            Some(p) => p,
            None => return,
        };

        let base_species = pokemon.base_species.clone();
        let tera_type = pokemon.tera_type.clone().unwrap_or_else(|| "Normal".to_string());
        let has_illusion = pokemon.illusion.is_some();

        (base_species, tera_type, has_illusion)
    };

    // if (pokemon.species.baseSpecies === 'Ogerpon' && !['Fire', 'Grass', 'Rock', 'Water'].includes(pokemon.teraType) &&
    //     (!pokemon.illusion || pokemon.illusion.species.baseSpecies === 'Ogerpon')) {
    if base_species.as_str() == "ogerpon" {
        let valid_types = ["Fire", "Grass", "Rock", "Water"];
        if !valid_types.contains(&tera_type.as_str()) {
            // this.battle.hint("If Ogerpon Terastallizes into a type other than Fire, Grass, Rock, or Water, the game softlocks.");
            battle.hint("If Ogerpon Terastallizes into a type other than Fire, Grass, Rock, or Water, the game softlocks.");
            return;
        }
    }

    // if (pokemon.illusion && ['Ogerpon', 'Terapagos'].includes(pokemon.illusion.species.baseSpecies)) {
    //     this.battle.singleEvent('End', this.dex.abilities.get('Illusion'), pokemon.abilityState, pokemon);
    // }
    // TODO: Implement illusion ending for Ogerpon/Terapagos

    // const type = pokemon.teraType;
    let tera_type_clone = tera_type.clone();

    // this.battle.add('-terastallize', pokemon, type);
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(side_index, pokemon_index) {
            Some(p) => p,
            None => return,
        };
        format!("p{}a: {}", side_index + 1, pokemon.set.species)
    };
    battle.add("-terastallize", &[
        crate::battle::Arg::String(pokemon_ident),
        crate::battle::Arg::String(tera_type_clone.clone()),
    ]);

    // pokemon.terastallized = type;
    if let Some(pokemon) = battle.pokemon_at_mut(side_index, pokemon_index) {
        pokemon.terastallized = Some(tera_type_clone.clone());
    }

    // for (const ally of pokemon.side.pokemon) {
    //     ally.canTerastallize = null;
    // }
    for i in 0..battle.sides[side_index].pokemon.len() {
        battle.sides[side_index].pokemon[i].can_terastallize = None;
    }

    // pokemon.addedType = '';
    // pokemon.knownType = true;
    // pokemon.apparentType = type;
    if let Some(pokemon) = battle.pokemon_at_mut(side_index, pokemon_index) {
        pokemon.added_type = String::new();
        pokemon.known_type = true;
        pokemon.apparent_type = Some(tera_type_clone.clone());
    }

    // if (pokemon.species.baseSpecies === 'Ogerpon') {
    //     let ogerponSpecies = toID(pokemon.species.battleOnly || pokemon.species.id);
    //     ogerponSpecies += ogerponSpecies === 'ogerpon' ? 'tealtera' : 'tera';
    //     pokemon.formeChange(ogerponSpecies, null, true);
    // }
    if base_species.as_str() == "ogerpon" {
        // TODO: Implement Ogerpon forme change
        // This requires accessing pokemon.species.battleOnly which may not exist in Rust struct
    }

    // if (pokemon.species.name === 'Terapagos-Terastal') {
    //     pokemon.formeChange('Terapagos-Stellar', null, true);
    // }
    let species_name = {
        let pokemon = match battle.pokemon_at(side_index, pokemon_index) {
            Some(p) => p,
            None => return,
        };
        pokemon.set.species.clone()
    };
    if species_name == "Terapagos-Terastal" {
        // TODO: Implement Terapagos-Stellar forme change
        // This requires implementing formeChange method
    }

    // if (pokemon.species.baseSpecies === 'Morpeko' && !pokemon.transformed &&
    //     pokemon.baseSpecies.id !== pokemon.species.id
    // ) {
    //     pokemon.formeRegression = true;
    //     pokemon.baseSpecies = pokemon.species;
    //     pokemon.details = pokemon.getUpdatedDetails();
    // }
    // TODO: Implement Morpeko special case
    // This requires Pokemon.transformed, formeRegression, and getUpdatedDetails

    // this.battle.runEvent('AfterTerastallization', pokemon);
    battle.run_event("AfterTerastallization", Some(pokemon_pos), None, None, None);
}
