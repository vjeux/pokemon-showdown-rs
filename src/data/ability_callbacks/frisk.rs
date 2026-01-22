//! Frisk Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onStart(pokemon) {
///     for (const target of pokemon.foes()) {
///         if (target.item) {
///             this.add('-item', target, target.getItem().name, '[from] ability: Frisk', `[of] ${pokemon}`);
///         }
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    use crate::battle::Arg;

    // Get pokemon identifier for "[of] ${pokemon}"
    let pokemon_id = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    // Get side index of the pokemon
    let side_index = pokemon_pos.0;
    let foe_side_index = if side_index == 0 { 1 } else { 0 };

    // for (const target of pokemon.foes())
    // Iterate through all active pokemon on the opposing side
    let foe_positions: Vec<(usize, usize)> = battle
        .get_all_active(false)
        .into_iter()
        .filter(|&pos| pos.0 == foe_side_index)
        .collect();

    for foe_pos in foe_positions {
        // if (target.item)
        let (has_item, item_name) = {
            let foe = match battle.pokemon_at(foe_pos.0, foe_pos.1) {
                Some(p) => p,
                None => continue,
            };
            let has_item = foe.item.as_str() != "";
            let item_name = foe.item.to_string();
            (has_item, item_name)
        };

        if !has_item {
            continue;
        }

        let foe_id = {
            let foe = match battle.pokemon_at(foe_pos.0, foe_pos.1) {
                Some(p) => p,
                None => continue,
            };
            foe.get_slot()
        };

        // this.add('-item', target, target.getItem().name, '[from] ability: Frisk', `[of] ${pokemon}`);
        battle.add("-item", &[
            Arg::String(foe_id),
            Arg::String(item_name),
            Arg::Str("[from] ability: Frisk"),
            Arg::String(format!("[of] {}", pokemon_id)),
        ]);
    }

    EventResult::Continue
}

