//! Pursuit Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     // You can't get here unless the pursuit succeeds
///     if (target.beingCalledBack || target.switchFlag) {
///         this.debug('Pursuit damage boost');
///         return move.basePower * 2;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => match battle.pokemon_at(pos.0, pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        },
        None => return EventResult::Continue,
    };

    // Get the active move
    let active_move = match &battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // You can't get here unless the pursuit succeeds
    // if (target.beingCalledBack || target.switchFlag)
    if target.being_called_back || target.switch_flag {
        let bp = active_move.base_power * 2;
        // this.debug('Pursuit damage boost');
        battle.debug("Pursuit damage boost");
        return EventResult::Number(bp);
    }

    EventResult::Number(active_move.base_power)
}

/// beforeTurnCallback(pokemon) {
///     for (const side of this.sides) {
///         if (side.hasAlly(pokemon)) continue;
///         side.addSideCondition('pursuit', pokemon);
///         const data = side.getSideConditionData('pursuit');
///         if (!data.sources) {
///             data.sources = [];
///         }
///         data.sources.push(pokemon);
///     }
/// }
pub fn before_turn_callback(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: This callback needs infrastructure support for side condition sources
    // The TypeScript version calls side.addSideCondition('pursuit', pokemon) where pokemon is the source
    // But the Rust add_side_condition signature is: fn add_side_condition(&mut self, id: ID, duration: Option<i32>)
    // It doesn't support passing a source pokemon parameter
    // Additionally, there's no getSideConditionData equivalent to manage the sources array in effect state
    // This needs infrastructure changes to support source tracking in side conditions
    EventResult::Continue
}

/// onModifyMove(move, source, target) {
///     if (target?.beingCalledBack || target?.switchFlag) move.accuracy = true;
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target?.beingCalledBack || target?.switchFlag) move.accuracy = true;
    let should_always_hit = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.being_called_back || target_pokemon.switch_flag
    };

    if should_always_hit {
        // move.accuracy = true;
        if let Some(ref mut active_move) = battle.active_move {
            active_move.accuracy = 0; // true in JS means always hit, represented as 0 in Rust
        }
    }

    EventResult::Continue
}

/// onTryHit(target, pokemon) {
///     target.side.removeSideCondition('pursuit');
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    let target = target_pos;

    // target.side.removeSideCondition('pursuit');
    let target_side = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.side_index
    };

    battle.sides[target_side].remove_side_condition(&ID::from("pursuit"));

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onBeforeSwitchOut(pokemon) {
    ///     this.debug('Pursuit start');
    ///     let alreadyAdded = false;
    ///     pokemon.removeVolatile('destinybond');
    ///     for (const source of this.effectState.sources) {
    ///         if (!source.isAdjacent(pokemon) || !this.queue.cancelMove(source) || !source.hp) continue;
    ///         if (!alreadyAdded) {
    ///             this.add('-activate', pokemon, 'move: Pursuit');
    ///             alreadyAdded = true;
    ///         }
    ///         // Run through each action in queue to check if the Pursuit user is supposed to Mega Evolve this turn.
    ///         // If it is, then Mega Evolve before moving.
    ///         if (source.canMegaEvo || source.canUltraBurst || source.canTerastallize) {
    ///             for (const [actionIndex, action] of this.queue.entries()) {
    ///                 if (action.pokemon === source) {
    ///                     if (action.choice === 'megaEvo') {
    ///                         this.actions.runMegaEvo(source);
    ///                     } else if (action.choice === 'terastallize') {
    ///                         // Also a "forme" change that happens before moves, though only possible in NatDex
    ///                         this.actions.terastallize(source);
    ///                     } else {
    ///                         continue;
    ///                     }
    ///                     this.queue.list.splice(actionIndex, 1);
    ///                     break;
    ///                 }
    ///             }
    ///         }
    ///         this.actions.runMove('pursuit', source, source.getLocOf(pokemon));
    ///     }
    /// }
    pub fn on_before_switch_out(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: This callback needs infrastructure support for side condition sources
        // The TypeScript version accesses this.effectState.sources which would be populated by beforeTurnCallback
        // However, as noted above, the infrastructure for side condition sources doesn't exist yet
        // This callback also needs:
        // - battle.debug() method
        // - queue.cancelMove() method
        // - actions.runMegaEvo() method
        // - actions.terastallize() method
        // - actions.runMove() method
        // This needs significant infrastructure changes
        EventResult::Continue
    }
}
