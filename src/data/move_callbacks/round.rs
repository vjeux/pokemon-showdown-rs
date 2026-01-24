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
pub fn base_power_callback(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (move.sourceEffect === 'round') {
    //     this.debug('BP doubled');
    //     return move.basePower * 2;
    // }
    // return move.basePower;
    let active_move = match &battle.active_move {
        Some(active_move) => active_move,
        None => return EventResult::Continue,
    };

    let base_power = active_move.borrow().base_power;
    // JavaScript sets move.sourceEffect = sourceEffect.id (a string ID)
    // In Rust, this is stored in source_effect_name
    let is_round_source = active_move
        .borrow()
        .source_effect_name
        .as_ref()
        .map(|se| se == "round")
        .unwrap_or(false);

    if is_round_source {
        battle.debug("BP doubled");
        return EventResult::Number(base_power * 2);
    }

    EventResult::Number(base_power)
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
pub fn on_try(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;
    use crate::battle::Effect;

    // for (const action of this.queue.list as MoveAction[]) {
    //     if (!action.pokemon || !action.move || action.maxMove || action.zmove) continue;
    //     if (action.move.id === 'round') {
    //         this.queue.prioritizeAction(action, move);
    //         return;
    //     }
    // }
    let queue_list = battle.queue.list.clone();
    let active_move_id = match &battle.active_move {
        Some(active_move) => active_move.borrow().id.clone(),
        None => return EventResult::Continue,
    };

    for action in queue_list.iter() {
        match action {
            crate::battle_queue::Action::Move(move_action) => {
                // Skip if maxMove or zmove
                if move_action.max_move.is_some() || move_action.zmove.is_some() {
                    continue;
                }

                // Check if move is 'round'
                if move_action.move_id == ID::from("round") {
                    // JavaScript: this.queue.prioritizeAction(action, move);
                    // The second parameter (move) is the current move being executed,
                    // which sets sourceEffect on the prioritized action.
                    // This is what triggers the base power doubling in basePowerCallback.
                    let source_effect = Effect::move_(active_move_id.clone());
                    battle.queue.prioritize_action_with_source(
                        move_action.side_index,
                        move_action.pokemon_index,
                        Some(source_effect),
                    );
                    return EventResult::Continue;
                }
            }
            _ => continue,
        }
    }

    EventResult::Continue
}
