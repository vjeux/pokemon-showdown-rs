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
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // return !!this.queue.willAct();
    let will_act = battle.queue.will_act();
    EventResult::Bool(will_act)
}

pub mod condition {
    use super::*;

    /// onSideStart(target, source) {
    ///     this.add('-singleturn', source, 'Crafty Shield');
    /// }
    pub fn on_side_start(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
        // this.add('-singleturn', source, 'Crafty Shield');
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source_arg = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(source_pokemon)
        };

        battle.add("-singleturn", &[source_arg, "Crafty Shield".into()]);

        EventResult::Continue
    }

    /// onTryHit(target, source, move) {
    ///     if (['self', 'all'].includes(move.target) || move.category !== 'Status') return;
    ///     this.add('-activate', target, 'move: Crafty Shield');
    ///     return this.NOT_FAIL;
    /// }
    pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        // if (['self', 'all'].includes(move.target) || move.category !== 'Status') return;
        // We need to get the current move being used
        // TODO: The move should be passed as a parameter, but for now we'll try to get it from context
        // For now, we'll implement a simplified version that checks if there's a current move

        // Get move data from battle context
        let move_id = match &battle.current_move {
            Some(id) => id.clone(),
            None => return EventResult::Continue,
        };

        let move_data = match battle.dex.get_move_by_id(&move_id) {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // if (['self', 'all'].includes(move.target) || move.category !== 'Status') return;
        if move_data.target == "self" || move_data.target == "all" || move_data.category != "Status" {
            // return;
            return EventResult::Continue;
        }

        // this.add('-activate', target, 'move: Crafty Shield');
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };

        battle.add("-activate", &[target_arg, "move: Crafty Shield".into()]);

        // return this.NOT_FAIL;
        // In Rust, NOT_FAIL is typically represented as a specific EventResult variant
        // For now, we'll return a special value that indicates the move should not fail
        EventResult::Bool(true)
    }
}
