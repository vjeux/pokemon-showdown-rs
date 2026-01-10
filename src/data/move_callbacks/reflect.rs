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
    pub fn duration_callback(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        _effect_id: Option<&str>,
    ) -> EventResult {
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
                source_pokemon.has_item(battle, &["lightclay"])
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
    pub fn on_any_modify_damage(
        battle: &mut Battle,
        _damage: i32,
        source_pos: Option<(usize, usize)>,
        target_pos: Option<(usize, usize)>,
        active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        use crate::dex_data::ID;
        let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");

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
            let effect_target = match battle.with_effect_state_ref(|state| state.target).flatten() {
                Some(t) => t,
                None => return EventResult::Continue,
            };

            let has_ally = battle.is_ally(effect_target, target);

            let category = {
                let move_data = match battle.dex.moves().get_by_id(&ID::from(move_id)) {
                    Some(m) => m,
                    None => return EventResult::Continue,
                };
                move_data.category.clone()
            };

            if has_ally && category == "Physical" {
                // if (!target.getMoveHitData(move).crit && !move.infiltrates) {
                // Use Battle::get_move_hit_data to get the actual stored crit value
                // (Pokemon::get_move_hit_data is deprecated and always returns default)
                let crit = battle
                    .get_move_hit_data(target)
                    .map(|hit_data| hit_data.crit)
                    .unwrap_or(false);

                let infiltrates = {
                    let active_move = match &battle.active_move {
                        Some(active_move) => active_move,
                        None => return EventResult::Continue,
                    };
                    active_move.infiltrates
                };

                if !crit && !infiltrates {
                    // this.debug('Reflect weaken');
                    battle.debug("Reflect weaken");
                    // if (this.activePerHalf > 1) return this.chainModify([2732, 4096]);
                    if battle.active_per_half > 1 {
                        battle.chain_modify_fraction(2732, 4096); return EventResult::Continue;
                    }
                    // return this.chainModify(0.5);
                    battle.chain_modify_fraction(1, 2); return EventResult::Continue;
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
        let side = match battle.with_effect_state_ref(|state| state.side).flatten() {
            Some(s) => s,
            None => return EventResult::Continue,
        };

        let side_arg = crate::battle::Arg::Str(if side == 0 { "p1" } else { "p2" });

        battle.add("-sidestart", &[side_arg, "Reflect".into()]);

        EventResult::Continue
    }

    /// onSideEnd(side) {
    ///     this.add('-sideend', side, 'Reflect');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', side, 'Reflect');
        let side = match battle.with_effect_state_ref(|state| state.side).flatten() {
            Some(s) => s,
            None => return EventResult::Continue,
        };

        let side_arg = crate::battle::Arg::Str(if side == 0 { "p1" } else { "p2" });

        battle.add("-sideend", &[side_arg, "Reflect".into()]);

        EventResult::Continue
    }
}
