//! G-Max Steelsurge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

pub mod condition {
    use super::*;

    /// onSideStart(side) {
    ///     this.add('-sidestart', side, 'move: G-Max Steelsurge');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', side, 'move: G-Max Steelsurge');
        if let Some(side_index) = battle.with_effect_state_ref(|state| state.side).flatten() {
            let side_id = if side_index == 0 { "p1" } else { "p2" };

            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "move: G-Max Steelsurge".into()]);
        }

        EventResult::Continue
    }

    /// onSwitchIn(pokemon) {
    ///     if (pokemon.hasItem('heavydutyboots')) return;
    ///     // Ice Face and Disguise correctly get typed damage from Stealth Rock
    ///     // because Stealth Rock bypasses Substitute.
    ///     // They don't get typed damage from Steelsurge because Steelsurge doesn't,
    ///     // so we're going to test the damage of a Steel-type Stealth Rock instead.
    ///     const steelHazard = this.dex.getActiveMove('Stealth Rock');
    ///     steelHazard.type = 'Steel';
    ///     const typeMod = this.clampIntRange(pokemon.runEffectiveness(steelHazard), -6, 6);
    ///     this.damage(pokemon.maxhp * (2 ** typeMod) / 8);
    /// }
    pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // if (pokemon.hasItem('heavydutyboots')) return;
        let has_heavy_duty_boots = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.has_item(battle, &["heavydutyboots"])
        };

        if has_heavy_duty_boots {
            return EventResult::Continue;
        }

        // const steelHazard = this.dex.getActiveMove('Stealth Rock');
        // steelHazard.type = 'Steel';
        // const typeMod = this.clampIntRange(pokemon.runEffectiveness(steelHazard), -6, 6);
        // Create a Steel-type Stealth Rock move for effectiveness calculation
        let mut steel_hazard = match battle.dex.get_active_move("stealthrock") {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        steel_hazard.move_type = "Steel".to_string();

        // Run effectiveness using the proper event system (matches JS pokemon.runEffectiveness())
        let raw_type_mod = Pokemon::run_effectiveness(battle, pokemon, &steel_hazard);
        let type_mod = battle.clamp_int_range(raw_type_mod, Some(-6), Some(6));

        // this.damage(pokemon.maxhp * (2 ** typeMod) / 8);
        // JavaScript uses floating-point math here, so 1 * (2 ** 0) / 8 = 0.125
        // In spreadDamage, non-zero values get clamped to at least 1:
        //   if (targetDamage !== 0) targetDamage = this.clampIntRange(targetDamage, 1);
        // So 0.125 becomes 1 in JavaScript. We must replicate this behavior.
        let damage_float = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            // JavaScript: pokemon.maxhp * (2 ** typeMod) / 8
            let maxhp = pokemon_pokemon.maxhp as f64;
            let multiplier = 2f64.powi(type_mod);
            maxhp * multiplier / 8.0
        };

        // Match JavaScript's clampIntRange behavior: non-zero values become at least 1
        let damage_amount = if damage_float > 0.0 {
            std::cmp::max(damage_float.floor() as i32, 1)
        } else {
            0
        };

        battle.damage(damage_amount, Some(pokemon), None, None, false);

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
    ///                   side.addSideCondition("gmaxsteelsurge");
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
        //     side.addSideCondition("gmaxsteelsurge");
        // }

        let source_side_idx = source_pos.0;

        // Get list of foe side indices
        // Note: foeSidesWithConditions() returns all foe sides, not just those with existing conditions
        let foe_side_indices: Vec<usize> = {
            let source_side = &battle.sides[source_side_idx];
            battle.sides
                .iter()
                .enumerate()
                .filter(|(idx, _side)| *idx != source_side.n)
                .map(|(idx, _)| idx)
                .collect()
        };

        // Add gmaxsteelsurge side condition to each foe side with conditions
        for foe_side_idx in foe_side_indices {
            let condition_id = ID::new("gmaxsteelsurge");
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
