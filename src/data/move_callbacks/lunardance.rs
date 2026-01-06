//! Lunar Dance Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

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
    let can_switch = battle.can_switch(source.0);

    if can_switch == 0 {
        //     this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        //     this.add('-fail', source);
        let source_arg = {
            let pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,

                None => return EventResult::Continue,
            };

            pokemon.get_slot()
        };
        battle.add("-fail", &[source_arg.into()]);

        //     return this.NOT_FAIL;
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

        battle.single_event("Swap", &effect_id, Some(target), Some(target), None, None);

        EventResult::Continue
    }

    /// onSwap(target) {
    ///     if (
    ///         !target.fainted && (
    ///             target.hp < target.maxhp ||
    ///             target.status ||
    ///             target.moveSlots.some(moveSlot => moveSlot.pp < moveSlot.maxpp)
    ///         )
    ///     ) {
    ///         target.heal(target.maxhp);
    ///         target.clearStatus();
    ///         for (const moveSlot of target.moveSlots) {
    ///             moveSlot.pp = moveSlot.maxpp;
    ///         }
    ///         this.add('-heal', target, target.getHealth, '[from] move: Lunar Dance');
    ///         target.side.removeSlotCondition(target, 'lunardance');
    ///     }
    /// }
    pub fn on_swap(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (
        //     !target.fainted && (
        //         target.hp < target.maxhp ||
        //         target.status ||
        //         target.moveSlots.some(moveSlot => moveSlot.pp < moveSlot.maxpp)
        //     )
        // )
        let should_heal = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            if target_pokemon.fainted {
                false
            } else {
                target_pokemon.hp < target_pokemon.maxhp
                    || target_pokemon.status != ID::from("")
                    || target_pokemon.move_slots.iter().any(|ms| ms.pp < ms.maxpp)
            }
        };

        if should_heal {
            //     target.heal(target.maxhp);
            let maxhp = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.maxhp
            };
            battle.heal(maxhp, Some(target), None, None);

            //     target.clearStatus();
            Pokemon::clear_status(battle, target);

            //     for (const moveSlot of target.moveSlots) {
            //         moveSlot.pp = moveSlot.maxpp;
            //     }
            {
                let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                for move_slot in &mut target_pokemon.move_slots {
                    move_slot.pp = move_slot.maxpp;
                }
            }

            //     this.add('-heal', target, target.getHealth, '[from] move: Lunar Dance');
            let health = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.get_health()
            };
            let target_arg = {
                let pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,

                    None => return EventResult::Continue,
                };

                pokemon.get_slot()
            };
            battle.add(
                "-heal",
                &[
                    target_arg.into(),
                    health.into(),
                    "[from] move: Lunar Dance".into(),
                ],
            );

            //     target.side.removeSlotCondition(target, 'lunardance');
            if let Some(target_side) = battle.sides.get_mut(target.0) {
                target_side.remove_slot_condition(target.1, &ID::from("lunardance"));
            }
        }

        EventResult::Continue
    }
}
