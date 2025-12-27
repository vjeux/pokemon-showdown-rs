//! Lucky Chant Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onSideStart(side) {
    ///     this.add('-sidestart', side, 'move: Lucky Chant'); // "The Lucky Chant shielded [side.name]'s team from critical hits!"
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', side, 'move: Lucky Chant');
        let side_index = match &battle.effect_state {
            Some(es) => es.target.0,
            None => return EventResult::Continue,
        };

        let side_arg = crate::battle::Arg::Side(side_index);
        battle.add("-sidestart", &[side_arg, "move: Lucky Chant".into()]);

        EventResult::Continue
    }

    /// onSideEnd(side) {
    ///     this.add('-sideend', side, 'move: Lucky Chant'); // "[side.name]'s team's Lucky Chant wore off!"
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', side, 'move: Lucky Chant');
        let side_index = match &battle.effect_state {
            Some(es) => es.target.0,
            None => return EventResult::Continue,
        };

        let side_arg = crate::battle::Arg::Side(side_index);
        battle.add("-sideend", &[side_arg, "move: Lucky Chant".into()]);

        EventResult::Continue
    }
}
