//! BattleActions::canTerastallize - Check if Pokemon can Terastallize
//!
//! 1:1 port of canTerastallize from battle-actions.ts

use crate::*;

/// Check if Pokemon can Terastallize
/// Equivalent to canTerastallize() in battle-actions.ts
///
/// JavaScript (battle-actions.ts):
///   canTerastallize(pokemon: Pokemon) {
///     if (pokemon.getItem().zMove || pokemon.canMegaEvo || this.dex.gen !== 9) {
///       return null;
///     }
///     return pokemon.teraType;
///   }
pub fn can_terastallize(
    battle: &Battle,
    pokemon_pos: (usize, usize),
) -> Option<String> {
    // Extract pokemon data
    let (item_id, tera_type, gen) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return None,
        };

        (pokemon.item.clone(), pokemon.tera_type.clone(), battle.gen)
    };

    // if (pokemon.getItem().zMove || pokemon.canMegaEvo || this.dex.gen !== 9) {
    //   return null;
    // }

    // Check if item is a Z-move
    let item_is_z_move = if !item_id.is_empty() {
        if let Some(item_data) = battle.dex.items.get(&item_id) {
            // Check if item has zMove property
            item_data.extra.get("zMove").is_some()
        } else {
            false
        }
    } else {
        false
    };

    // Check if pokemon can Mega Evolve
    let can_mega_evo = crate::battle_actions::can_mega_evo(battle, pokemon_pos.0, pokemon_pos.1).is_some();

    // Check generation and other conditions
    if item_is_z_move || can_mega_evo || gen != 9 {
        return None;
    }

    // return pokemon.teraType;
    tera_type
}
