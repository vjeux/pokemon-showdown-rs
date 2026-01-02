//! Quick Guard Move
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
    // Use battle.add_volatile_to_pokemon to properly set duration from dex.conditions
    Pokemon::add_volatile(battle, source, ID::from("stall"), None, None);

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onSideStart(target, source) {
    ///     this.add('-singleturn', source, 'Quick Guard');
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

        // this.add('-singleturn', source, 'Quick Guard');
        let source_arg = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.get_slot()
        };

        battle.add(
            "-singleturn",
            &[source_arg.into(), "Quick Guard".into()],
        );

        EventResult::Continue
    }

    /// onTryHit(target, source, move) {
    ///     // Quick Guard blocks moves with positive priority, even those given increased priority by Prankster or Gale Wings.
    ///     // (e.g. it blocks 0 priority moves boosted by Prankster or Gale Wings; Quick Claw/Custap Berry do not count)
    ///     if (move.priority <= 0.1) return;
    ///     if (!move.flags['protect']) {
    ///         if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
    ///         if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
    ///         return;
    ///     }
    ///     this.add('-activate', target, 'move: Quick Guard');
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
        let move_id = match &battle.active_move {
            Some(active_move) => active_move.id.clone(),
            None => return EventResult::Continue,
        };

        let move_data = match battle.dex.moves().get_by_id(&move_id) {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // Get move priority from active_move
        let move_priority = match &battle.active_move {
            Some(active_move) => active_move.priority,
            None => return EventResult::Continue,
        };

        // if (move.priority <= 0.1) return;
        if move_priority as f64 <= 0.1 {
            return EventResult::Continue;
        }

        // if (!move.flags['protect']) {
        if !move_data.flags.contains_key("protect") {
            // if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
            if move_id.as_str() == "gmaxoneblow" || move_id.as_str() == "gmaxrapidflow" {
                return EventResult::Continue;
            }

            // if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
            if move_data.is_z.is_some() || move_data.is_max.is_some() {
                // TODO: getMoveHitData not yet implemented to set zBrokeProtect
            }

            return EventResult::Continue;
        }

        // this.add('-activate', target, 'move: Quick Guard');
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-activate",
            &[target_ident.as_str().into(), "move: Quick Guard".into()],
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
