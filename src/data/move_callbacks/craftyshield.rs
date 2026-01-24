//! Crafty Shield Move
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

pub mod condition {
    use super::*;

    /// onSideStart(target, source) {
    ///     this.add('-singleturn', source, 'Crafty Shield');
    /// }
    pub fn on_side_start(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // this.add('-singleturn', source, 'Crafty Shield');
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source_ident = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.get_slot()
        };

        battle.add(
            "-singleturn",
            &[source_ident.as_str().into(), "Crafty Shield".into()],
        );

        EventResult::Continue
    }

    /// onTryHit(target, source, move) {
    ///     if (['self', 'all'].includes(move.target) || move.category !== 'Status') return;
    ///     this.add('-activate', target, 'move: Crafty Shield');
    ///     return this.NOT_FAIL;
    /// }
    pub fn on_try_hit(
        battle: &mut Battle,
        _source_pos: (usize, usize),
        target_pos: (usize, usize),
        _move_id: Option<&str>,
    ) -> EventResult {
        // Get move data from battle context
        let move_id = match &battle.active_move {
            Some(active_move) => active_move.borrow().id.clone(),
            None => return EventResult::Continue,
        };

        let move_data = match battle.dex.moves().get_by_id(&move_id) {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // if (['self', 'all'].includes(move.target) || move.category !== 'Status') return;
        if move_data.target == "self" || move_data.target == "all" || move_data.category != "Status"
        {
            return EventResult::Continue;
        }

        // this.add('-activate', target, 'move: Crafty Shield');
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-activate",
            &[target_ident.as_str().into(), "move: Crafty Shield".into()],
        );

        // return this.NOT_FAIL;
        EventResult::NotFail
    }
}
