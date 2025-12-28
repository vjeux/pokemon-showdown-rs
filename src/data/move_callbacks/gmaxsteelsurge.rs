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
                let side_arg = crate::battle::Arg::Side(side_index);
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
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // if (pokemon.hasItem('heavydutyboots')) return;
        let has_heavy_duty_boots = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.has_item(&ID::from("heavydutyboots"))
        };

        if has_heavy_duty_boots {
            return EventResult::Continue;
        }

        // const steelHazard = this.dex.getActiveMove('Stealth Rock');
        // steelHazard.type = 'Steel';
        // const typeMod = this.clampIntRange(pokemon.runEffectiveness(steelHazard), -6, 6);
        // We need to run effectiveness as if using a Steel-type Stealth Rock
        // For now, we'll use the stealth rock move ID but the effectiveness system
        // should handle type-based calculations
        let type_mod = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            // Run effectiveness for Steel-type Stealth Rock
            // The move is Stealth Rock but we need to treat it as Steel type
            let effectiveness = pokemon_pokemon.run_effectiveness_for_type(&ID::from("steel"), battle);
            // Clamp between -6 and 6
            effectiveness.max(-6).min(6)
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

        battle.damage(damage_amount, pokemon, None, None);

        EventResult::Continue
    }
}
