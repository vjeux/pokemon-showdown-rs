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

        // Get the side from the effect context (Effect.side_index), not EffectState.side
        let effect_side_index = battle.effect.as_ref().and_then(|e| e.side_index);

        // this.add('-sidestart', side, 'Spikes');
        let side_index = battle.with_effect_state_ref(|state| state.side).flatten();

        if let Some(side_idx) = effect_side_index.or(side_index) {
            let side_id = if side_idx == 0 { "p1" } else { "p2" };
            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "Spikes".into()]);
        }

        // this.effectState.layers = 1;
        battle.with_effect_state(|state| {
            state.layers = Some(1);
        });

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

        // Get the side from the effect context
        let effect_side_index = battle.effect.as_ref().and_then(|e| e.side_index);

        // if (this.effectState.layers >= 3) return false;
        let layers = battle
            .with_effect_state_ref(|state| state.layers.unwrap_or(0))
            .unwrap_or(0);

        if layers >= 3 {
            return EventResult::Boolean(false);
        }

        // this.add('-sidestart', side, 'Spikes');
        let side_index = battle.with_effect_state_ref(|state| state.side).flatten();

        if let Some(side_idx) = effect_side_index.or(side_index) {
            let side_id = if side_idx == 0 { "p1" } else { "p2" };
            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "Spikes".into()]);
        }

        // this.effectState.layers++;
        let new_layers = layers + 1;
        battle.with_effect_state(|state| {
            state.layers = Some(new_layers);
        });

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
            pokemon_pokemon.is_grounded(battle, false).unwrap_or(false)
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
        let layers = battle.with_effect_state_ref(|state| {
            state.layers.unwrap_or(1)
        }).unwrap_or(1);
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
