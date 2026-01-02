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
        let side_index = battle.current_effect_state.as_ref().and_then(|es| es.side);

        if let Some(side_idx) = side_index {
            let side_id = if side_idx == 0 { "p1" } else { "p2" };
            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "Spikes".into()]);
        }

        // this.effectState.layers = 1;
        if let Some(effect_state) = &mut battle.current_effect_state {
            effect_state
                .data
                .insert("layers".to_string(), serde_json::json!(1));
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
        let layers = battle
            .current_effect_state
            .as_ref()
            .and_then(|es| es.data.get("layers"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;

        if layers >= 3 {
            return EventResult::Boolean(false);
        }

        // this.add('-sidestart', side, 'Spikes');
        let side_index = battle.current_effect_state.as_ref().and_then(|es| es.side);

        if let Some(side_idx) = side_index {
            let side_id = if side_idx == 0 { "p1" } else { "p2" };
            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "Spikes".into()]);
        }

        // this.effectState.layers++;
        if let Some(effect_state) = &mut battle.current_effect_state {
            effect_state
                .data
                .insert("layers".to_string(), serde_json::json!(layers + 1));
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
        let is_grounded = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.is_grounded(battle)
        };
        let has_heavy_duty_boots = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.has_item(battle, &["heavydutyboots"])
        };

        if !is_grounded || has_heavy_duty_boots {
            return EventResult::Continue;
        }

        // const damageAmounts = [0, 3, 4, 6]; // 1/8, 1/6, 1/4
        // this.damage(damageAmounts[this.effectState.layers] * pokemon.maxhp / 24);
        let layers = match &battle.current_effect_state {
            Some(state) => state
                .data
                .get("layers")
                .and_then(|v| v.as_i64())
                .unwrap_or(1) as i32,
            None => 1,
        };
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
