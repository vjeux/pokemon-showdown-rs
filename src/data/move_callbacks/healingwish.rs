//! Healing Wish Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(source) {
///     if (!this.canSwitch(source.side)) {
///         this.attrLastMove('[still]');
///         this.add('-fail', source);
///         return this.NOT_FAIL;
///     }
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    let source = source_pos;

    // if (!this.canSwitch(source.side)) {
    let source_side = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.side_index
    };

    let can_switch = battle.can_switch(source_side);

    if can_switch == 0 {
        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // this.add('-fail', source);
        let source_arg = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.get_slot()
        };

        battle.add("-fail", &[source_arg.into()]);

        // return this.NOT_FAIL;
        return EventResult::NotFail;
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onSwitchIn(target) {
    ///     this.singleEvent('Swap', this.effect, this.effectState, target);
    /// }
    pub fn on_switch_in(_battle: &mut Battle, _target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onSwap(target) {
    ///     if (!target.fainted && (target.hp < target.maxhp || target.status)) {
    ///         target.heal(target.maxhp);
    ///         target.clearStatus();
    ///         this.add('-heal', target, target.getHealth, '[from] move: Healing Wish');
    ///         target.side.removeSlotCondition(target, 'healingwish');
    ///     }
    /// }
    pub fn on_swap(_battle: &mut Battle, _target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
