//! Light Screen Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// durationCallback(target, source, effect) {
    ///     if (source?.hasItem('lightclay')) {
    ///         return 8;
    ///     }
    ///     return 5;
    /// }
    pub fn duration_callback(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        _effect: Option<&Effect>,
    ) -> EventResult {
        // if (source?.hasItem('lightclay')) {
        //     return 8;
        // }
        let has_light_clay = if let Some(source) = source_pos {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Number(5),
            };
            source_pokemon.has_item(battle, &["lightclay"])
        } else {
            false
        };

        if has_light_clay {
            return EventResult::Number(8);
        }

        // return 5;
        EventResult::Number(5)
    }

    /// onAnyModifyDamage(damage, source, target, move) {
    ///     if (target !== source && this.effectState.target.hasAlly(target) && this.getCategory(move) === 'Special') {
    ///         if (!target.getMoveHitData(move).crit && !move.infiltrates) {
    ///             this.debug('Light Screen weaken');
    ///             if (this.activePerHalf > 1) return this.chainModify([2732, 4096]);
    ///             return this.chainModify(0.5);
    ///         }
    ///     }
    /// }
    pub fn on_any_modify_damage(
        battle: &mut Battle,
        _damage: i32,
        source_pos: Option<(usize, usize)>,
        target_pos: Option<(usize, usize)>,
        active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        let _move_id = active_move.map(|m| m.id.to_string()).unwrap_or_default();
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (target !== source && this.effectState.target.hasAlly(target) && this.getCategory(move) === 'Special') {
        // target !== source
        if target == source {
            return EventResult::Continue;
        }

        // this.effectState.target.hasAlly(target)
        let effect_state_target = match battle.with_effect_state_ref(|state| state.target).flatten() {
            Some(t) => t,
            None => return EventResult::Continue,
        };

        let has_ally = battle.is_ally(effect_state_target, target);
        if !has_ally {
            return EventResult::Continue;
        }

        // this.getCategory(move) === 'Special'
        let is_special = {
            if let Some(ref active_move) = battle.active_move {
                active_move.borrow().category == "Special"
            } else {
                false
            }
        };

        if !is_special {
            return EventResult::Continue;
        }

        //     if (!target.getMoveHitData(move).crit && !move.infiltrates) {
        // Use Battle::get_move_hit_data to get the actual stored crit value
        // (Pokemon::get_move_hit_data is deprecated and always returns default)
        let is_crit = battle
            .get_move_hit_data(target)
            .map(|hit_data| hit_data.crit)
            .unwrap_or(false);

        let infiltrates = battle
            .active_move
            .as_ref()
            .map(|m| m.borrow().infiltrates)
            .unwrap_or(false);

        if is_crit || infiltrates {
            return EventResult::Continue;
        }

        //         this.debug('Light Screen weaken');
        battle.debug("Light Screen weaken");

        //         if (this.activePerHalf > 1) return this.chainModify([2732, 4096]);
        //         return this.chainModify(0.5);
        if battle.active_per_half > 1 {
            { battle.chain_modify_fraction(2732, 4096); EventResult::Continue }
        } else {
            { battle.chain_modify(0.5_f32); EventResult::Continue }
        }
    }

    /// onSideStart(side) {
    ///     this.add('-sidestart', side, 'move: Light Screen');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', side, 'move: Light Screen');
        let side_index = match battle.with_effect_state_ref(|state| state.target).flatten() {
            Some((side, _)) => side,
            None => return EventResult::Continue,
        };

        let side_id = if side_index == 0 { "p1" } else { "p2" };

        let side_arg = crate::battle::Arg::Str(side_id);
        battle.add("-sidestart", &[side_arg, "move: Light Screen".into()]);

        EventResult::Continue
    }

    /// onSideEnd(side) {
    ///     this.add('-sideend', side, 'move: Light Screen');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', side, 'move: Light Screen');
        let side_index = match battle.with_effect_state_ref(|state| state.target).flatten() {
            Some((side, _)) => side,
            None => return EventResult::Continue,
        };

        let side_id = if side_index == 0 { "p1" } else { "p2" };

        let side_arg = crate::battle::Arg::Str(side_id);
        battle.add("-sideend", &[side_arg, "move: Light Screen".into()]);

        EventResult::Continue
    }
}
