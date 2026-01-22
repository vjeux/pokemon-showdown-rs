//! Punching Glove Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onBasePower(basePower, attacker, defender, move) {
///     if (move.flags['punch']) {
///         this.debug('Punching Glove boost');
///         return this.chainModify([4506, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the active move flags
    let is_punch = match &battle.active_move {
        Some(active_move) => active_move.flags.punch,
        None => return EventResult::Continue,
    };

    // if (move.flags['punch'])
    if is_punch {
        // this.debug('Punching Glove boost');
        battle.debug("Punching Glove boost");
        // return this.chainModify([4506, 4096]);
        battle.chain_modify_fraction(4506, 4096);
        return EventResult::Continue;
    }

    EventResult::Continue
}

/// onModifyMove(move) {
///     if (move.flags['punch']) delete move.flags['contact'];
/// }
pub fn on_modify_move(battle: &mut Battle, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // Get mutable reference to active move
    if let Some(ref mut active_move) = battle.active_move {
        // if (move.flags['punch'])
        if active_move.flags.punch {
            // delete move.flags['contact'];
            active_move.flags.contact = false;
        }
    }

    EventResult::Continue
}
