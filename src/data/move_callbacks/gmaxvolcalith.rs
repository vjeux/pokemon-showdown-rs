//! G-Max Volcalith Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onSideStart(targetSide) {
    ///     this.add('-sidestart', targetSide, 'G-Max Volcalith');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', targetSide, 'G-Max Volcalith');
        if let Some(side_index) = battle.with_effect_state_ref(|state| state.side).flatten() {
            let side_id = if side_index == 0 { "p1" } else { "p2" };

            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "G-Max Volcalith".into()]);
        }

        EventResult::Continue
    }

    /// onResidual(target) {
    ///     if (!target.hasType('Rock')) this.damage(target.baseMaxhp / 6, target);
    /// }
    pub fn on_residual(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        eprintln!("[GMAXVOLCALITH::CONDITION::ON_RESIDUAL] Called with target_pos={:?}", target_pos);
        let target = match target_pos {
            Some(pos) => pos,
            None => {
                eprintln!("[GMAXVOLCALITH::CONDITION::ON_RESIDUAL] No target, returning Continue");
                return EventResult::Continue;
            }
        };

        // if (!target.hasType('Rock')) this.damage(target.baseMaxhp / 6, target);
        let has_rock_type = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => {
                    eprintln!("[GMAXVOLCALITH::CONDITION::ON_RESIDUAL] No pokemon at {:?}, returning Continue", target);
                    return EventResult::Continue;
                }
            };
            let result = target_pokemon.has_type(battle, "rock");
            eprintln!("[GMAXVOLCALITH::CONDITION::ON_RESIDUAL] has_rock_type={}", result);
            result
        };

        if !has_rock_type {
            // this.damage(target.baseMaxhp / 6, target);
            let damage_amount = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.base_maxhp / 6
            };

            eprintln!("[GMAXVOLCALITH::CONDITION::ON_RESIDUAL] Dealing {} damage to {:?}", damage_amount, target);
            battle.damage(damage_amount, Some(target), None, None, false);
        }

        eprintln!("[GMAXVOLCALITH::CONDITION::ON_RESIDUAL] Returning Continue");
        EventResult::Continue
    }

    /// onSideEnd(targetSide) {
    ///     this.add('-sideend', targetSide, 'G-Max Volcalith');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', targetSide, 'G-Max Volcalith');
        if let Some(side_index) = battle.with_effect_state_ref(|state| state.side).flatten() {
            let side_id = if side_index == 0 { "p1" } else { "p2" };

            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sideend", &[side_arg, "G-Max Volcalith".into()]);
        }

        EventResult::Continue
    }
}

/// Self-targeting callbacks
/// These callbacks target the move user (source), not the move target
pub mod self_callbacks {
    use super::*;

    /// self.onHit(source)
    ///
    /// ```text
    /// JS Source (data/moves.ts):
    /// self: {
    ///     onHit(source) {
    ///         onHit(source) {
    ///                 for (const side of source.side.foeSidesWithConditions()) {
    ///                   side.addSideCondition("gmaxvolcalith");
    ///                 }
    ///               }
    ///     },
    /// }
    /// ```
    pub fn on_hit(
        battle: &mut Battle,
        _target_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        eprintln!("[GMAXVOLCALITH::SELF::ON_HIT] Called with source_pos={:?}", source_pos);
        // for (const side of source.side.foeSidesWithConditions()) {
        //     side.addSideCondition("gmaxvolcalith");
        // }

        let source = match source_pos {
            Some(pos) => pos,
            None => {
                eprintln!("[GMAXVOLCALITH::SELF::ON_HIT] No source, returning Continue");
                return EventResult::Continue;
            }
        };

        let source_side_idx = source.0;
        eprintln!("[GMAXVOLCALITH::SELF::ON_HIT] source_side_idx={}", source_side_idx);

        // Get foe sides (in singles, just the opposite side)
        for side_idx in 0..battle.sides.len() {
            if side_idx != source_side_idx {
                // This is a foe side, add the condition
                eprintln!("[GMAXVOLCALITH::SELF::ON_HIT] Adding gmaxvolcalith to side {}", side_idx);
                let condition_id = crate::dex_data::ID::new("gmaxvolcalith");
                battle.add_side_condition(
                    side_idx,
                    condition_id,
                    Some(source),
                    None,
                );
                eprintln!("[GMAXVOLCALITH::SELF::ON_HIT] Added gmaxvolcalith to side {}", side_idx);
            }
        }

        eprintln!("[GMAXVOLCALITH::SELF::ON_HIT] Returning Continue");
        EventResult::Continue
    }
}
