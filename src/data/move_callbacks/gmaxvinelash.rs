//! G-Max Vine Lash Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onSideStart(targetSide) {
    ///     this.add('-sidestart', targetSide, 'G-Max Vine Lash');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', targetSide, 'G-Max Vine Lash');
        if let Some(side_index) = battle.with_effect_state_ref(|state| state.side).flatten() {
            let side_id = if side_index == 0 { "p1" } else { "p2" };

            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "G-Max Vine Lash".into()]);
        }

        EventResult::Continue
    }

    /// onResidual(target) {
    ///     if (!target.hasType('Grass')) this.damage(target.baseMaxhp / 6, target);
    /// }
    pub fn on_residual(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (!target.hasType('Grass')) this.damage(target.baseMaxhp / 6, target);
        let has_grass_type = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.has_type(battle, "grass")
        };

        if !has_grass_type {
            // this.damage(target.baseMaxhp / 6, target);
            let damage_amount = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.base_maxhp / 6
            };

            battle.damage(damage_amount, Some(target), None, None, false);
        }

        EventResult::Continue
    }

    /// onSideEnd(targetSide) {
    ///     this.add('-sideend', targetSide, 'G-Max Vine Lash');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', targetSide, 'G-Max Vine Lash');
        if let Some(side_index) = battle.with_effect_state_ref(|state| state.side).flatten() {
            let side_id = if side_index == 0 { "p1" } else { "p2" };

            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sideend", &[side_arg, "G-Max Vine Lash".into()]);
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
    ///                   side.addSideCondition("gmaxvinelash");
    ///                 }
    ///               }
    ///     },
    /// }
    /// ```
    ///
    /// NOTE: For self callbacks, the FIRST parameter receives the move USER (source),
    /// and the SECOND parameter receives the move TARGET (or None).
    /// The naming convention in dispatch_self_on_hit is misleading - it names them
    /// target_pos and source_pos, but actually passes source as first, target as second.
    pub fn on_hit(
        battle: &mut Battle,
        source_pos: (usize, usize),          // ACTUAL SOURCE (move user)
        _target_pos: Option<(usize, usize)>, // ACTUAL TARGET (move target)
        _source_effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        use crate::dex_data::ID;

        // for (const side of source.side.foeSidesWithConditions()) {
        //     side.addSideCondition("gmaxvinelash");
        // }

        let source_side_idx = source_pos.0;

        // Get list of foe side indices
        // Note: foeSidesWithConditions() returns all foe sides, not just those with existing conditions
        let foe_side_indices: Vec<usize> = {
            let _source_side = &battle.sides[source_side_idx];
            battle.sides
                .iter()
                .enumerate()
                .filter(|(idx, _side)| *idx != source_side_idx)
                .map(|(idx, _)| idx)
                .collect()
        };

        // Add gmaxvinelash side condition to each foe side with conditions
        for foe_side_idx in foe_side_indices {
            let condition_id = ID::new("gmaxvinelash");
            battle.add_side_condition(
                foe_side_idx,
                condition_id,
                Some(source_pos),
                None,
            );
        }

        EventResult::Continue
    }
}
