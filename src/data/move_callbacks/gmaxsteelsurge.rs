//! G-Max Steelsurge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onSideStart(side) {
    ///     this.add('-sidestart', side, 'move: G-Max Steelsurge');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', side, 'move: G-Max Steelsurge');
        if let Some(effect_state) = &battle.current_effect_state {
            if let Some(side_index) = effect_state.side {
                let side_id = if side_index == 0 { "p1" } else { "p2" };

                let side_arg = crate::battle::Arg::Str(side_id);
                battle.add("-sidestart", &[side_arg, "move: G-Max Steelsurge".into()]);
            }
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
        // We need to run effectiveness as if using a Steel-type Stealth Rock
        use crate::dex_data::ID;

        // Create a Steel-type move ID for effectiveness calculation
        let _steel_move_id = ID::from("stealthrock"); // Using Stealth Rock as base move

        // Run effectiveness for Steel-type (the move type is checked in run_effectiveness)
        // Note: In full implementation, we'd modify battle.active_move.move_type to "Steel" temporarily
        // For now, we'll use a direct Steel type effectiveness check
        let type_mod = {
            // Use the Dex to get Steel type effectiveness against pokemon's types
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let pokemon_types = pokemon_ref.get_types(battle, false);
            let mut total_mod = 0;
            for poke_type in &pokemon_types {
                total_mod += battle.dex.get_effectiveness("Steel", poke_type);
            }
            // Clamp between -6 and 6
            battle.clamp_int_range(total_mod, Some(-6), Some(6))
        };

        // this.damage(pokemon.maxhp * (2 ** typeMod) / 8);
        let damage_amount = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let power = if type_mod >= 0 {
                2i32.pow(type_mod as u32)
            } else {
                // For negative type mods, we use 1/2^abs(typeMod)
                // which is equivalent to dividing by 2^abs(typeMod)
                1
            };

            if type_mod >= 0 {
                pokemon_pokemon.maxhp * power / 8
            } else {
                pokemon_pokemon.maxhp / (2i32.pow((-type_mod) as u32) * 8)
            }
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
    pub fn on_hit(
        battle: &mut Battle,
        _target_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
