//! Wide Guard Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry() {
///     return !!this.queue.willAct();
/// }
pub fn on_try(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // return !!this.queue.willAct();
    let will_act = battle.queue.will_act().is_some();
    EventResult::Boolean(will_act)
}

/// onHitSide(side, source) {
///     source.addVolatile('stall');
/// }
pub fn on_hit_side(battle: &mut Battle, source_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;
    use crate::pokemon::Pokemon;

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // source.addVolatile('stall');
    Pokemon::add_volatile(battle, source, ID::from("stall"), None);

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onSideStart(target, source) {
    ///     this.add('-singleturn', source, 'Wide Guard');
    /// }
    pub fn on_side_start(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-singleturn', source, 'Wide Guard');
        let source_slot = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.get_slot()
        };

        battle.add(
            "-singleturn",
            &[
                crate::battle::Arg::from(source_slot),
                crate::battle::Arg::from("Wide Guard"),
            ],
        );

        EventResult::Continue
    }

    /// onTryHit(target, source, move) {
    ///     // Wide Guard blocks all spread moves
    ///     if (move?.target !== 'allAdjacent' && move.target !== 'allAdjacentFoes') {
    ///         return;
    ///     }
    ///     if (move.isZ || move.isMax) {
    ///         if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
    ///         target.getMoveHitData(move).zBrokeProtect = true;
    ///         return;
    ///     }
    ///     this.add('-activate', target, 'move: Wide Guard');
    ///     const lockedmove = source.getVolatile('lockedmove');
    ///     if (lockedmove) {
    ///         // Outrage counter is reset
    ///         if (source.volatiles['lockedmove'].duration === 2) {
    ///             delete source.volatiles['lockedmove'];
    ///         }
    ///     }
    ///     return this.NOT_FAIL;
    /// }
    pub fn on_try_hit(
        battle: &mut Battle,
        source_pos: (usize, usize),
        target_pos: (usize, usize),
    ) -> EventResult {
        use crate::dex_data::ID;

        let source = source_pos;
        let target = target_pos;

        // Get the active move
        let move_data = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // if (move?.target !== 'allAdjacent' && move.target !== 'allAdjacentFoes') {
        //     return;
        // }
        if move_data.target != "allAdjacent" && move_data.target != "allAdjacentFoes" {
            return EventResult::Continue;
        }

        // if (move.isZ || move.isMax) {
        if move_data.is_z || move_data.is_max {
            // if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
            if move_data.id == ID::from("gmaxoneblow") || move_data.id == ID::from("gmaxrapidflow") {
                return EventResult::Continue;
            }

            // target.getMoveHitData(move).zBrokeProtect = true;
            // TODO: getMoveHitData().zBrokeProtect not yet implemented
            // return;
            return EventResult::Continue;
        }

        // this.add('-activate', target, 'move: Wide Guard');
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-activate",
            &[target_ident.as_str().into(), "move: Wide Guard".into()],
        );

        // const lockedmove = source.getVolatile('lockedmove');
        // if (lockedmove) {
        //     // Outrage counter is reset
        //     if (source.volatiles['lockedmove'].duration === 2) {
        //         delete source.volatiles['lockedmove'];
        //     }
        // }
        let lockedmove_id = ID::from("lockedmove");
        let should_remove_lockedmove = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            if let Some(volatile) = source_pokemon.volatiles.get(&lockedmove_id) {
                volatile.duration == Some(2)
            } else {
                false
            }
        };

        if should_remove_lockedmove {
            let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.remove_volatile(&lockedmove_id);
        }

        // return this.NOT_FAIL;
        EventResult::NotFail
    }
}
