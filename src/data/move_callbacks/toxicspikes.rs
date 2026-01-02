//! Toxic Spikes Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onSideStart(side) {
    ///     this.add('-sidestart', side, 'move: Toxic Spikes');
    ///     this.effectState.layers = 1;
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', side, 'move: Toxic Spikes');
        let side_index = battle.current_effect_state.as_ref().and_then(|es| es.side);

        if let Some(side_idx) = side_index {
            let side_id = if side_idx == 0 { "p1" } else { "p2" };
            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "move: Toxic Spikes".into()]);
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
    ///     if (this.effectState.layers >= 2) return false;
    ///     this.add('-sidestart', side, 'move: Toxic Spikes');
    ///     this.effectState.layers++;
    /// }
    pub fn on_side_restart(battle: &mut Battle) -> EventResult {
        // if (this.effectState.layers >= 2) return false;
        let layers = battle
            .current_effect_state
            .as_ref()
            .and_then(|es| es.data.get("layers"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;

        if layers >= 2 {
            return EventResult::Boolean(false);
        }

        // this.add('-sidestart', side, 'move: Toxic Spikes');
        let side_index = battle.current_effect_state.as_ref().and_then(|es| es.side);

        if let Some(side_idx) = side_index {
            let side_id = if side_idx == 0 { "p1" } else { "p2" };
            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "move: Toxic Spikes".into()]);
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
    ///     if (!pokemon.isGrounded()) return;
    ///     if (pokemon.hasType('Poison')) {
    ///         this.add('-sideend', pokemon.side, 'move: Toxic Spikes', `[of] ${pokemon}`);
    ///         pokemon.side.removeSideCondition('toxicspikes');
    ///     } else if (pokemon.hasType('Steel') || pokemon.hasItem('heavydutyboots')) {
    ///         // do nothing
    ///     } else if (this.effectState.layers >= 2) {
    ///         pokemon.trySetStatus('tox', pokemon.side.foe.active[0]);
    ///     } else {
    ///         pokemon.trySetStatus('psn', pokemon.side.foe.active[0]);
    ///     }
    /// }
    pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // if (!pokemon.isGrounded()) return;
        let is_grounded = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.is_grounded(battle, false)
        };

        if !is_grounded {
            return EventResult::Continue;
        }

        // if (pokemon.hasType('Poison'))
        let has_poison_type = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.has_type(battle, "Poison")
        };

        if has_poison_type {
            // this.add('-sideend', pokemon.side, 'move: Toxic Spikes', `[of] ${pokemon}`);
            let (side_id, pokemon_slot) = {
                let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                let side_id = if pokemon.0 == 0 { "p1" } else { "p2" };
                (side_id, pokemon_ref.get_slot())
            };

            battle.add(
                "-sideend",
                &[
                    crate::battle::Arg::from(side_id),
                    crate::battle::Arg::from("move: Toxic Spikes"),
                    crate::battle::Arg::from(format!("[of] {}", pokemon_slot)),
                ],
            );

            // pokemon.side.removeSideCondition('toxicspikes');
            let side = &mut battle.sides[pokemon.0];
            side.remove_side_condition(&ID::from("toxicspikes"));
        } else {
            // else if (pokemon.hasType('Steel') || pokemon.hasItem('heavydutyboots'))
            let (has_steel_type, has_heavy_duty_boots) = {
                let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                (
                    pokemon_ref.has_type(battle, "Steel"),
                    pokemon_ref.has_item(battle, &["heavydutyboots"]),
                )
            };

            if has_steel_type || has_heavy_duty_boots {
                // do nothing
                return EventResult::Continue;
            }

            // else if (this.effectState.layers >= 2)
            let layers = battle
                .current_effect_state
                .as_ref()
                .and_then(|es| es.data.get("layers"))
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;

            if layers >= 2 {
                // pokemon.trySetStatus('tox', pokemon.side.foe.active[0]);
                let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon_mut.try_set_status(ID::from("tox"), Some("Toxic Spikes"));
            } else {
                // pokemon.trySetStatus('psn', pokemon.side.foe.active[0]);
                let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon_mut.try_set_status(ID::from("psn"), Some("Toxic Spikes"));
            }
        }

        EventResult::Continue
    }
}
