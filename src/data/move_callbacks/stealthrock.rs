//! Stealth Rock Move
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
    ///     this.add('-sidestart', side, 'move: Stealth Rock');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // onSideStart(side) {
        //     this.add('-sidestart', side, 'move: Stealth Rock');
        // }

        // this.add('-sidestart', side, 'move: Stealth Rock');
        let side_index = battle.with_effect_state_ref(|state| state.side).flatten();
        if let Some(side_idx) = side_index {
            let side_id = if side_idx == 0 { "p1" } else { "p2" };
            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "move: Stealth Rock".into()]);
        }

        EventResult::Continue
    }

    /// onSwitchIn(pokemon) {
    ///     if (pokemon.hasItem('heavydutyboots')) return;
    ///     const typeMod = this.clampIntRange(pokemon.runEffectiveness(this.dex.getActiveMove('stealthrock')), -6, 6);
    ///     this.damage(pokemon.maxhp * (2 ** typeMod) / 8);
    /// }
    pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // onSwitchIn(pokemon) {
        //     if (pokemon.hasItem('heavydutyboots')) return;
        //     const typeMod = this.clampIntRange(pokemon.runEffectiveness(this.dex.getActiveMove('stealthrock')), -6, 6);
        //     this.damage(pokemon.maxhp * (2 ** typeMod) / 8);
        // }
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

        // const typeMod = this.clampIntRange(pokemon.runEffectiveness(this.dex.getActiveMove('stealthrock')), -6, 6);
        // Get the active move for stealthrock to properly run effectiveness through the event system
        let active_move = match battle.dex.get_active_move("stealthrock") {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // Run effectiveness using the proper event system (matches JS pokemon.runEffectiveness())
        let raw_type_mod = Pokemon::run_effectiveness(battle, pokemon, &active_move);
        let type_mod = battle.clamp_int_range(raw_type_mod, Some(-6), Some(6));

        // this.damage(pokemon.maxhp * (2 ** typeMod) / 8);
        let max_hp = {
            let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_data.maxhp
        };

        // JavaScript uses 2 ** typeMod which handles negative exponents as fractions
        // e.g., 2 ** -2 = 0.25, so maxhp * 0.25 / 8 = maxhp / 32
        // IMPORTANT: JavaScript uses floating point math here, so 1 * 2 / 8 = 0.25
        // In spreadDamage, non-zero values get clamped to at least 1:
        //   if (targetDamage !== 0) targetDamage = this.clampIntRange(targetDamage, 1);
        // So 0.25 becomes 1 in JavaScript. We must replicate this behavior.
        let damage_float = if type_mod >= 0 {
            // Positive: multiply by 2^typeMod
            let multiplier = 1_i32 << type_mod;
            (max_hp * multiplier) as f64 / 8.0
        } else {
            // Negative: divide by 2^(-typeMod)
            let divisor = 1_i32 << (-type_mod);
            max_hp as f64 / (8.0 * divisor as f64)
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
