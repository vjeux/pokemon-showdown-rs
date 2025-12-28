//! Round Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(target, source, move) {
///     if (move.sourceEffect === 'round') {
///         this.debug('BP doubled');
///         return move.basePower * 2;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // if (move.sourceEffect === 'round') {
    //     this.debug('BP doubled');
    //     return move.basePower * 2;
    // }
    // return move.basePower;
    let active_move = match &battle.active_move {
        Some(active_move) => &active_move.id,
        None => return EventResult::Continue,
    };

    if active_move.source_effect.as_ref().map(|se| se.id == ID::from("round")).unwrap_or(false) {
        battle.debug("BP doubled");
        return EventResult::Number(active_move.base_power * 2);
    }

    EventResult::Number(active_move.base_power)
}

/// onTry(source, target, move) {
///     for (const action of this.queue.list as MoveAction[]) {
///         if (!action.pokemon || !action.move || action.maxMove || action.zmove) continue;
///         if (action.move.id === 'round') {
///             this.queue.prioritizeAction(action, move);
///             return;
///         }
///     }
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // for (const action of this.queue.list as MoveAction[]) {
    //     if (!action.pokemon || !action.move || action.maxMove || action.zmove) continue;
    //     if (action.move.id === 'round') {
    //         this.queue.prioritizeAction(action, move);
    //         return;
    //     }
    // }
    let queue_list = battle.queue.list.clone();
    let active_move_id = {
        let active_move = match &battle.active_move {
            Some(active_move) => &active_move.id,
            None => return EventResult::Continue,
        };
        active_move.clone()
    };

    for (i, action) in queue_list.iter().enumerate() {
        if action.pokemon.is_none() || action.choice.is_none() {
            continue;
        }

        if action.max_move || action.z_move {
            continue;
        }

        if let Some(ref choice) = action.choice {
            if choice == "move" {
                if let Some(ref move_id) = action.move_id {
                    if move_id == &ID::from("round") {
                        battle.prioritize_action(i, &active_move_id);
                        return EventResult::Continue;
                    }
                }
            }
        }
    }

    EventResult::Continue
}

