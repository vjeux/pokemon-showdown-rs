//! BattleActions::runMegaEvo - Perform Mega Evolution
//!
//! 1:1 port of runMegaEvo from battle-actions.ts

use crate::*;
use crate::battle_actions::{can_mega_evo, can_ultra_burst};

/// Perform Mega Evolution or Ultra Burst
/// Equivalent to battle-actions.ts runMegaEvo()
///
/// runMegaEvo(pokemon: Pokemon) {
///     const speciesid = pokemon.canMegaEvo || pokemon.canUltraBurst;
///     if (!speciesid) return false;
///
///     pokemon.formeChange(speciesid, pokemon.getItem(), true);
///
///     // Limit one mega evolution
///     const wasMega = pokemon.canMegaEvo;
///     for (const ally of pokemon.side.pokemon) {
///         if (wasMega) {
///             ally.canMegaEvo = false;
///         } else {
///             ally.canUltraBurst = null;
///         }
///     }
///
///     this.battle.runEvent('AfterMega', pokemon);
///     return true;
/// }
pub fn run_mega_evo(
    battle: &mut Battle,
    side_index: usize,
    pokemon_index: usize,
) -> bool {
    // const speciesid = pokemon.canMegaEvo || pokemon.canUltraBurst;
    let speciesid: Option<String> = {
        if let Some(mega_species) = can_mega_evo(battle, side_index, pokemon_index) {
            Some(mega_species)
        } else {
            can_ultra_burst(battle, side_index, pokemon_index)
        }
    };

    // if (!speciesid) return false;
    let speciesid: String = match speciesid {
        Some(s) => s,
        None => return false,
    };

    // pokemon.formeChange(speciesid, pokemon.getItem(), true);
    // TODO: Current forme_change method doesn't match JavaScript signature
    // JavaScript: formeChange(speciesId: string | Species, source: Effect | null, isPermanent?: boolean, abilitySlot = '0', message?: string)
    // Current Rust: forme_change(&mut self, new_species_id: ID, new_types: Vec<String>, new_ability: Option<ID>)
    // For now, get the species data and call the simplified version
    let (species_id, types, ability) = {
        let species_data = battle.dex.species().get(&speciesid).cloned();
        match species_data {
            Some(sp) => {
                let types = sp.types.clone();
                let ability = sp.abilities.slot0.clone().map(|a| ID::from(a.as_str()));
                (ID::from(sp.name.as_str()), types, ability)
            }
            None => return false,
        }
    };

    battle.sides[side_index].pokemon[pokemon_index].forme_change(species_id, types, ability);

    // Limit one mega evolution
    // const wasMega = pokemon.canMegaEvo;
    let was_mega = battle.sides[side_index].pokemon[pokemon_index].can_mega_evo.is_some();

    // for (const ally of pokemon.side.pokemon) {
    //     if (wasMega) {
    //         ally.canMegaEvo = false;
    //     } else {
    //         ally.canUltraBurst = null;
    //     }
    // }
    let num_pokemon = battle.sides[side_index].pokemon.len();
    for i in 0..num_pokemon {
        if was_mega {
            battle.sides[side_index].pokemon[i].can_mega_evo = None;
        } else {
            battle.sides[side_index].pokemon[i].can_ultra_burst = None;
        }
    }

    // this.battle.runEvent('AfterMega', pokemon);
    battle.run_event("AfterMega", Some((side_index, pokemon_index)), None, None, None);

    // return true;
    true
}
