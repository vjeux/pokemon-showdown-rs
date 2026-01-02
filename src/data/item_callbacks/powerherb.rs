//! Power Herb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onChargeMove(pokemon, target, move) {
///     if (pokemon.useItem()) {
///         this.debug('power herb - remove charge turn for ' + move.id);
///         this.attrLastMove('[still]');
///         this.addMove('-anim', pokemon, move.name, target);
///         return false; // skip charge turn
///     }
/// }
pub fn on_charge_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    move_id: &str,
) -> EventResult {
    // if (pokemon.useItem())
    let used_item = {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_mut.use_item(None, None)
    };

    if used_item.is_some() {
        // this.debug('power herb - remove charge turn for ' + move.id);
        battle.debug(&format!("power herb - remove charge turn for {}", move_id));

        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // this.addMove('-anim', pokemon, move.name, target);
        let move_name = battle
            .dex
            .moves().get(move_id)
            .map(|m| m.name.clone())
            .unwrap_or_else(|| move_id.to_string());

        let pokemon_slot = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        if let Some(target) = target_pos {
            let target_slot = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.get_slot()
            };
            battle.add_move(&["-anim", &pokemon_slot, &move_name, &target_slot]);
        } else {
            battle.add_move(&["-anim", &pokemon_slot, &move_name]);
        }

        // return false; // skip charge turn
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}
