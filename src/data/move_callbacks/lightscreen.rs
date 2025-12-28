//! Light Screen Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// durationCallback(target, source, effect) {
    ///     if (source?.hasItem('lightclay')) {
    ///         return 8;
    ///     }
    ///     return 5;
    /// }
    pub fn duration_callback(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        use crate::dex_data::ID;

        // if (source?.hasItem('lightclay')) {
        //     return 8;
        // }
        let has_light_clay = if let Some(source) = source_pos {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Number(5),
            };
            source_pokemon.has_item(&["lightclay"])
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
    pub fn on_any_modify_damage(battle: &mut Battle, damage: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
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
        let effect_state_target = match &battle.current_effect_state {
            Some(es) => es.target,
            None => return EventResult::Continue,
        };

        let has_ally = battle.has_ally(effect_state_target, target);
        if !has_ally {
            return EventResult::Continue;
        }

        // this.getCategory(move) === 'Special'
        let is_special = {
            if let Some(ref active_move) = battle.active_move {
                active_move.category == crate::move_types::MoveCategory::Special
            } else {
                false
            }
        };

        if !is_special {
            return EventResult::Continue;
        }

        //     if (!target.getMoveHitData(move).crit && !move.infiltrates) {
        let (is_crit, infiltrates) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let move_id_id = crate::dex_data::ID::from(move_id);
            let is_crit = target_pokemon.get_move_hit_data(&move_id_id)
                .map(|hd| hd.crit)
                .unwrap_or(false);

            let infiltrates = battle.active_move.as_ref()
                .map(|m| m.infiltrates)
                .unwrap_or(false);

            (is_crit, infiltrates)
        };

        if is_crit || infiltrates {
            return EventResult::Continue;
        }

        //         this.debug('Light Screen weaken');
        battle.debug("Light Screen weaken");

        //         if (this.activePerHalf > 1) return this.chainModify([2732, 4096]);
        //         return this.chainModify(0.5);
        if battle.active_per_half > 1 {
            EventResult::Number(battle.chain_modify_fraction(2732, 4096))
        } else {
            EventResult::Number(battle.chain_modify(0.5 as f32))
        }
    }

    /// onSideStart(side) {
    ///     this.add('-sidestart', side, 'move: Light Screen');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', side, 'move: Light Screen');
        let side_index = match &battle.current_effect_state {
            Some(es) => match es.target {
                Some((side, _)) => side,
                None => return EventResult::Continue,
            },
            None => return EventResult::Continue,
        };

        let side_arg = crate::battle::Arg::Side(side_index);
        battle.add("-sidestart", &[side_arg, "move: Light Screen".into()]);

        EventResult::Continue
    }

    /// onSideEnd(side) {
    ///     this.add('-sideend', side, 'move: Light Screen');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', side, 'move: Light Screen');
        let side_index = match &battle.current_effect_state {
            Some(es) => match es.target {
                Some((side, _)) => side,
                None => return EventResult::Continue,
            },
            None => return EventResult::Continue,
        };

        let side_arg = crate::battle::Arg::Side(side_index);
        battle.add("-sideend", &[side_arg, "move: Light Screen".into()]);

        EventResult::Continue
    }
}
