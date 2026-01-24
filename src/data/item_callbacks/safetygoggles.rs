//! Safety Goggles Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onImmunity(type, pokemon) {
///     if (type === 'sandstorm' || type === 'hail' || type === 'powder') return false;
/// }
pub fn on_immunity(_battle: &mut Battle, _pokemon_pos: (usize, usize), immunity_type: &str) -> EventResult {
    // if (type === 'sandstorm' || type === 'hail' || type === 'powder') return false;
    if immunity_type == "sandstorm" || immunity_type == "hail" || immunity_type == "powder" {
        return EventResult::Boolean(false);
    }
    EventResult::Continue
}

/// onTryHit(pokemon, source, move) {
///     if (move.flags['powder'] && pokemon !== source && this.dex.getImmunity('powder', pokemon)) {
///         this.add('-activate', pokemon, 'item: Safety Goggles', move.name);
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    // if (move.flags['powder'] && pokemon !== source && this.dex.getImmunity('powder', pokemon))

    // Get active move
    let (has_powder_flag, move_name) = {
        if let Some(ref active_move) = battle.active_move {
            (active_move.borrow().flags.powder, active_move.borrow().name.clone())
        } else {
            return EventResult::Continue;
        }
    };

    // pokemon !== source
    if !has_powder_flag || target_pos == source_pos {
        return EventResult::Continue;
    }

    // this.dex.getImmunity('powder', pokemon)
    let has_immunity = {
        let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        // Use get_types to get the actual types (handles Arceus/Silvally forme changes)
        // JavaScript: dex.getImmunity('powder', pokemon) calls pokemon.getTypes()
        let types = pokemon.get_types(battle, false);
        battle.dex.get_immunity("powder", &types)
    };

    if has_immunity {
        // this.add('-activate', pokemon, 'item: Safety Goggles', move.name);
        let pokemon_slot = {
            let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add(
            "-activate",
            &[
                crate::battle::Arg::from(pokemon_slot),
                crate::battle::Arg::from("item: Safety Goggles"),
                crate::battle::Arg::from(move_name),
            ],
        );

        // return null;
        return EventResult::Null;
    }

    EventResult::Continue
}
