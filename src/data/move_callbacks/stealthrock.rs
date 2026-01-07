//! Stealth Rock Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

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
        // Run effectiveness for Rock-type Stealth Rock
        let type_mod = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let pokemon_types = pokemon_ref.get_types(battle, false);
            let mut total_mod = 0;
            for poke_type in &pokemon_types {
                total_mod += battle.dex.get_effectiveness("Rock", poke_type);
            }
            // Clamp between -6 and 6
            battle.clamp_int_range(total_mod, Some(-6), Some(6))
        };

        // this.damage(pokemon.maxhp * (2 ** typeMod) / 8);
        let max_hp = {
            let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_data.maxhp
        };

        let multiplier = 2_i32.pow(type_mod as u32);
        let damage_amount = (max_hp * multiplier) / 8;
        battle.damage(damage_amount, Some(pokemon), None, None, false);

        EventResult::Continue
    }
}
