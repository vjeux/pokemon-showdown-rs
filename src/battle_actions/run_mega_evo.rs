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
    // JavaScript: formeChange(speciesId, source, isPermanent, abilitySlot, message)
    // Get the item as source
    let item_id = {
        let pokemon = &battle.sides[side_index].pokemon[pokemon_index];
        pokemon.item.clone()
    };

    // Call the proper forme_change method (position-based to avoid borrow checker issues)
    let success = crate::pokemon::Pokemon::forme_change(
        battle,
        (side_index, pokemon_index),
        ID::from(speciesid.as_str()),
        Some(item_id), // source (item)
        true,          // is_permanent
        "0",           // ability_slot
        None,          // message
    );

    if !success {
        return false;
    }

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
