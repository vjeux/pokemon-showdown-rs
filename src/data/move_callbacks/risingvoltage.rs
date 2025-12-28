//! Rising Voltage Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(source, target, move) {
///     if (this.field.isTerrain('electricterrain') && target.isGrounded()) {
///         if (!source.isAlly(target)) this.hint(`${move.name}'s BP doubled on grounded target.`);
///         return move.basePower * 2;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (this.field.isTerrain('electricterrain') && target.isGrounded()) {
    let is_electric_terrain = battle.is_terrain(&ID::from("electricterrain"));
    let target_is_grounded = battle.is_grounded(target);

    if is_electric_terrain && target_is_grounded {
        // if (!source.isAlly(target)) this.hint(`${move.name}'s BP doubled on grounded target.`);
        let is_ally = battle.is_ally(source, target);

        if !is_ally {
            let move_name = {
                let active_move = match &battle.active_move {
                    Some(active_move) => &active_move.id,
                    None => return EventResult::Continue,
                };
                active_move.name.clone()
            };
            battle.hint(&format!("{}'s BP doubled on grounded target.", move_name));
        }

        // return move.basePower * 2;
        let base_power = {
            let active_move = match &battle.active_move {
                Some(active_move) => &active_move.id,
                None => return EventResult::Continue,
            };
            active_move.base_power
        };
        return EventResult::Number(base_power * 2);
    }

    // return move.basePower;
    let base_power = {
        let active_move = match &battle.active_move {
            Some(active_move) => &active_move.id,
            None => return EventResult::Continue,
        };
        active_move.base_power
    };

    EventResult::Number(base_power)
}

