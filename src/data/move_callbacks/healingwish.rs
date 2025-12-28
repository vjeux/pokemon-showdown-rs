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
    pub fn on_switch_in(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.singleEvent('Swap', this.effect, this.effectState, target);
        let effect_id = match &battle.current_effect_state {
            Some(es) => es.id.clone(),
            None => return EventResult::Continue,
        };

        battle.single_event("Swap", &effect_id, Some(target), Some(target), None);

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
    pub fn on_swap(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (!target.fainted && (target.hp < target.maxhp || target.status)) {
        let (is_fainted, hp, maxhp, has_status, side_index, slot) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (
                target_pokemon.fainted,
                target_pokemon.hp,
                target_pokemon.maxhp,
                !target_pokemon.status.is_empty(),
                target_pokemon.side_index,
                target_pokemon.get_slot(),
            )
        };

        if !is_fainted && (hp < maxhp || has_status) {
            // target.heal(target.maxhp);
            battle.heal(maxhp, Some(target), None, None);

            // target.clearStatus();
            {
                let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.clear_status();
            }

            // this.add('-heal', target, target.getHealth, '[from] move: Healing Wish');
            let health = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.get_health()
            };

            battle.add(
                "-heal",
                &[
                    crate::battle::Arg::from(slot),
                    crate::battle::Arg::from(health),
                    crate::battle::Arg::from("[from] move: Healing Wish"),
                ],
            );

            // target.side.removeSlotCondition(target, 'healingwish');
            let _ = battle.sides[side_index].remove_slot_condition(target.1, &ID::from("healingwish"));
        }

        EventResult::Continue
    }
}
