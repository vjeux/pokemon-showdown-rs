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
        let side = battle.get_effect_state_side();
        battle.add_side("-sidestart", side, &["move: Stealth Rock".into()]);

        EventResult::Continue
    }

    /// onSwitchIn(pokemon) {
    ///     if (pokemon.hasItem('heavydutyboots')) return;
    ///     const typeMod = this.clampIntRange(pokemon.runEffectiveness(this.dex.getActiveMove('stealthrock')), -6, 6);
    ///     this.damage(pokemon.maxhp * (2 ** typeMod) / 8);
    /// }
    pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        // onSwitchIn(pokemon) {
        //     if (pokemon.hasItem('heavydutyboots')) return;
        //     const typeMod = this.clampIntRange(pokemon.runEffectiveness(this.dex.getActiveMove('stealthrock')), -6, 6);
        //     this.damage(pokemon.maxhp * (2 ** typeMod) / 8);
        // }
        let pokemon = pokemon_pos;

        // if (pokemon.hasItem('heavydutyboots')) return;
        let has_heavy_duty_boots = battle.has_item(pokemon, "heavydutyboots");

        if has_heavy_duty_boots {
            return EventResult::Continue;
        }

        // const typeMod = this.clampIntRange(pokemon.runEffectiveness(this.dex.getActiveMove('stealthrock')), -6, 6);
        let effectiveness = battle.run_effectiveness(pokemon, &ID::from("stealthrock"));
        let type_mod = battle.clamp_int_range(effectiveness, Some(-6), Some(6));

        // this.damage(pokemon.maxhp * (2 ** typeMod) / 8);
        let max_hp = {
            let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_data.maxhp
        };

        let multiplier = 2_i32.pow(type_mod as u32);
        let damage_amount = (max_hp * multiplier as i32) / 8;
        battle.damage(damage_amount, Some(pokemon), None, None, false);

        EventResult::Continue
    }
}
