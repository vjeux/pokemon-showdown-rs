//! Assault Vest Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifySpD(spd) {
///     return this.chainModify(1.5);
/// }
pub fn on_modify_sp_d(battle: &mut Battle) -> EventResult {
    // return this.chainModify(1.5);
    battle.chain_modify(1.5);
    EventResult::Continue
}

/// onDisableMove(pokemon) {
///     for (const moveSlot of pokemon.moveSlots) {
///         const move = this.dex.moves.get(moveSlot.id);
///         if (move.category === 'Status' && move.id !== 'mefirst') {
///             pokemon.disableMove(moveSlot.id);
///         }
///     }
/// }
pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // for (const moveSlot of pokemon.moveSlots) {
    //     const move = this.dex.moves.get(moveSlot.id);
    //     if (move.category === 'Status' && move.id !== 'mefirst') {
        //         pokemon.disableMove(moveSlot.id);
    //     }
    // }

    // Get move slots to iterate over
    let move_ids: Vec<_> = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.move_slots.iter().map(|slot| slot.id.clone()).collect()
    };

    // Check each move and disable if it's a Status move (except Me First)
    for move_id in move_ids {
        if let Some(move_data) = battle.dex.moves().get_by_id(&move_id) {
            // if (move.category === 'Status' && move.id !== 'mefirst')
            if move_data.category == "Status" && move_id.as_str() != "mefirst" {
                // pokemon.disableMove(moveSlot.id);
                let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon_mut.disable_move(move_id.as_str(), false, Some("Assault Vest".to_string()));
            }
        }
    }

    EventResult::Continue
}
