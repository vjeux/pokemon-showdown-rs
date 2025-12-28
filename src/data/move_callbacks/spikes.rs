//! Spikes Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onSideStart(side) {
    ///     this.add('-sidestart', side, 'Spikes');
    ///     this.effectState.layers = 1;
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // onSideStart(side) {
        //     this.add('-sidestart', side, 'Spikes');
        //     this.effectState.layers = 1;
        // }

        // this.add('-sidestart', side, 'Spikes');
        if let Some(effect_state) = &mut battle.current_effect_state {
            if let Some(side_index) = effect_state.side {
                let side_id = if side_index == 0 { "p1" } else { "p2" };
                let side_arg = crate::battle::Arg::Str(side_id);
                battle.add("-sidestart", &[side_arg, "Spikes".into()]);
            }

            // this.effectState.layers = 1;
            effect_state.data.insert("layers".to_string(), serde_json::json!(1));
        }

        EventResult::Continue
    }

    /// onSideRestart(side) {
    ///     if (this.effectState.layers >= 3) return false;
    ///     this.add('-sidestart', side, 'Spikes');
    ///     this.effectState.layers++;
    /// }
    pub fn on_side_restart(battle: &mut Battle) -> EventResult {
        // onSideRestart(side) {
        //     if (this.effectState.layers >= 3) return false;
        //     this.add('-sidestart', side, 'Spikes');
        //     this.effectState.layers++;
        // }

        // if (this.effectState.layers >= 3) return false;
        if let Some(effect_state) = &mut battle.current_effect_state {
            let layers = effect_state.data.get("layers").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

            if layers >= 3 {
                return EventResult::Boolean(false);
            }

            // this.add('-sidestart', side, 'Spikes');
            if let Some(side_index) = effect_state.side {
                let side_id = if side_index == 0 { "p1" } else { "p2" };
                let side_arg = crate::battle::Arg::Str(side_id);
                battle.add("-sidestart", &[side_arg, "Spikes".into()]);
            }

            // this.effectState.layers++;
            effect_state.data.insert("layers".to_string(), serde_json::json!(layers + 1));
        }

        EventResult::Continue
    }

    /// onSwitchIn(pokemon) {
    ///     if (!pokemon.isGrounded() || pokemon.hasItem('heavydutyboots')) return;
    ///     const damageAmounts = [0, 3, 4, 6]; // 1/8, 1/6, 1/4
    ///     this.damage(damageAmounts[this.effectState.layers] * pokemon.maxhp / 24);
    /// }
    pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // onSwitchIn(pokemon) {
        //     if (!pokemon.isGrounded() || pokemon.hasItem('heavydutyboots')) return;
        //     const damageAmounts = [0, 3, 4, 6]; // 1/8, 1/6, 1/4
        //     this.damage(damageAmounts[this.effectState.layers] * pokemon.maxhp / 24);
        // }
        let pokemon = pokemon_pos;

        // if (!pokemon.isGrounded() || pokemon.hasItem('heavydutyboots')) return;
        let is_grounded = battle.is_grounded(pokemon);
        let has_heavy_duty_boots = battle.has_item(pokemon, "heavydutyboots");

        if !is_grounded || has_heavy_duty_boots {
            return EventResult::Continue;
        }

        // const damageAmounts = [0, 3, 4, 6]; // 1/8, 1/6, 1/4
        // this.damage(damageAmounts[this.effectState.layers] * pokemon.maxhp / 24);
        let layers = battle.get_effect_state_layers();
        let damage_amounts = [0, 3, 4, 6];

        let max_hp = {
            let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_data.maxhp
        };

        let damage_amount = damage_amounts[layers as usize] * max_hp / 24;
        battle.damage(damage_amount, Some(pokemon), None, None, false);

        EventResult::Continue
    }
}
