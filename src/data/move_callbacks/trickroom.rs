//! Trick Room Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// durationCallback(source, effect) {
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-activate', source, 'ability: Persistent', '[move] Trick Room');
    ///         return 7;
    ///     }
    ///     return 5;
    /// }
    pub fn duration_callback(
        _battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        _effect_id: Option<&str>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onFieldStart(target, source) {
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-fieldstart', 'move: Trick Room', `[of] ${source}`, '[persistent]');
    ///     } else {
    ///         this.add('-fieldstart', 'move: Trick Room', `[of] ${source}`);
    ///     }
    /// }
    pub fn on_field_start(
        _battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onFieldRestart(target, source) {
    ///     this.field.removePseudoWeather('trickroom');
    /// }
    pub fn on_field_restart(
        _battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Trick Room');
    /// }
    pub fn on_field_end(battle: &mut Battle) -> EventResult {
        // this.add('-fieldend', 'move: Trick Room');
        battle.add("-fieldend", &[crate::battle::Arg::from("move: Trick Room")]);

        EventResult::Continue
    }
}
