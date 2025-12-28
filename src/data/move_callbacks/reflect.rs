//! Reflect Move
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
        // return 5;
        if let Some(source) = source_pos {
            let has_light_clay = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.has_item(&ID::from("lightclay"))
            };

            if has_light_clay {
                return EventResult::Number(8);
            }
        }

        EventResult::Number(5)
    }

    /// onAnyModifyDamage(damage, source, target, move) {
    ///     if (target !== source && this.effectState.target.hasAlly(target) && this.getCategory(move) === 'Physical') {
    ///         if (!target.getMoveHitData(move).crit && !move.infiltrates) {
    ///             this.debug('Reflect weaken');
    ///             if (this.activePerHalf > 1) return this.chainModify([2732, 4096]);
    ///             return this.chainModify(0.5);
    ///         }
    ///     }
    /// }
    pub fn on_any_modify_damage(battle: &mut Battle, damage: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        use crate::dex_data::ID;

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (target !== source && this.effectState.target.hasAlly(target) && this.getCategory(move) === 'Physical') {
        if target != source {
            let effect_target = {
                let effect_state = match &battle.current_effect_state {
                    Some(es) => es,
                    None => return EventResult::Continue,
                };
                match effect_state.target {
                    Some(t) => t,
                    None => return EventResult::Continue,
                }
            };

            let has_ally = battle.has_ally(effect_target, target);

            let category = {
                let move_data = match battle.dex.get_move_by_id(&ID::from(move_id)) {
                    Some(m) => m,
                    None => return EventResult::Continue,
                };
                move_data.category.clone()
            };

            if has_ally && category == "Physical" {
                // if (!target.getMoveHitData(move).crit && !move.infiltrates) {
                let crit = battle.get_move_hit_data(target, move_id).crit;
                let infiltrates = {
                    let active_move = match &battle.active_move {
                        Some(active_move) => &active_move.id,
                        None => return EventResult::Continue,
                    };
                    active_move.infiltrates
                };

                if !crit && !infiltrates {
                    // this.debug('Reflect weaken');
                    battle.debug("Reflect weaken");
                    // if (this.activePerHalf > 1) return this.chainModify([2732, 4096]);
                    if battle.active_per_half > 1 {
                        return EventResult::Number(battle.chain_modify_fraction(2732, 4096));
                    }
                    // return this.chainModify(0.5);
                    return EventResult::Number(battle.chain_modify_fraction(1, 2));
                }
            }
        }

        EventResult::Continue
    }

    /// onSideStart(side) {
    ///     this.add('-sidestart', side, 'Reflect');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', side, 'Reflect');
        let side = {
            let effect_state = match &battle.current_effect_state {
                Some(es) => es,
                None => return EventResult::Continue,
            };
            match effect_state.side {
                Some(s) => s,
                None => return EventResult::Continue,
            }
        };

        let side_arg = crate::battle::Arg::from_side(battle, side);

        battle.add("-sidestart", &[
            side_arg,
            "Reflect".into(),
        ]);

        EventResult::Continue
    }

    /// onSideEnd(side) {
    ///     this.add('-sideend', side, 'Reflect');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', side, 'Reflect');
        let side = {
            let effect_state = match &battle.current_effect_state {
                Some(es) => es,
                None => return EventResult::Continue,
            };
            match effect_state.side {
                Some(s) => s,
                None => return EventResult::Continue,
            }
        };

        let side_arg = crate::battle::Arg::from_side(battle, side);

        battle.add("-sideend", &[
            side_arg,
            "Reflect".into(),
        ]);

        EventResult::Continue
    }
}
