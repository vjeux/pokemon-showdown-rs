//! Sweet Veil Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAllySetStatus(status, target, source, effect) {
///     if (status.id === 'slp') {
///         this.debug('Sweet Veil interrupts sleep');
///         const effectHolder = this.effectState.target;
///         this.add('-block', target, 'ability: Sweet Veil', `[of] ${effectHolder}`);
///         return null;
///     }
/// }
pub fn on_ally_set_status(battle: &mut Battle, status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;

    // if (status.id === 'slp')
    if status_id == "slp" {
        // this.debug('Sweet Veil interrupts sleep');
        // Note: Debug logging not implemented yet

        // const effectHolder = this.effectState.target;
        // this.add('-block', target, 'ability: Sweet Veil', `[of] ${effectHolder}`);
        let target_slot = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Null,
            };
            target.get_slot()
        };

        battle.add("-block", &[
            Arg::String(target_slot),
            Arg::Str("ability: Sweet Veil"),
            // TODO: Add [of] ${effectHolder} when effectState.target is available
        ]);

        // return null;
        return EventResult::Null;
    }

    EventResult::Continue
}

/// onAllyTryAddVolatile(status, target) {
///     if (status.id === 'yawn') {
///         this.debug('Sweet Veil blocking yawn');
///         const effectHolder = this.effectState.target;
///         this.add('-block', target, 'ability: Sweet Veil', `[of] ${effectHolder}`);
///         return null;
///     }
/// }
pub fn on_ally_try_add_volatile(battle: &mut Battle, status: Option<&str>, target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::battle::Arg;

    let status_id = match status {
        Some(s) => s,
        None => return EventResult::Continue,
    };

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (status.id === 'yawn')
    if status_id == "yawn" {
        // this.debug('Sweet Veil blocking yawn');
        // Note: Debug logging not implemented yet

        // const effectHolder = this.effectState.target;
        // this.add('-block', target, 'ability: Sweet Veil', `[of] ${effectHolder}`);
        let target_slot = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Null,
            };
            target.get_slot()
        };

        battle.add("-block", &[
            Arg::String(target_slot),
            Arg::Str("ability: Sweet Veil"),
            // TODO: Add [of] ${effectHolder} when effectState.target is available
        ]);

        // return null;
        return EventResult::Null;
    }

    EventResult::Continue
}

