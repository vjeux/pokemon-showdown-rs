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
        let side = battle.get_effect_state_side();
        battle.add_side("-sidestart", side, &["Spikes".into()]);

        // this.effectState.layers = 1;
        battle.set_effect_state_layers(1);

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
        let layers = battle.get_effect_state_layers();
        if layers >= 3 {
            return EventResult::Boolean(false);
        }

        // this.add('-sidestart', side, 'Spikes');
        let side = battle.get_effect_state_side();
        battle.add_side("-sidestart", side, &["Spikes".into()]);

        // this.effectState.layers++;
        battle.set_effect_state_layers(layers + 1);

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
        battle.damage(damage_amount, pokemon);

        EventResult::Continue
    }
}
