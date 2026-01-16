//! Toxic Spikes Move
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
    ///     this.add('-sidestart', side, 'move: Toxic Spikes');
    ///     this.effectState.layers = 1;
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', side, 'move: Toxic Spikes');
        let side_index = battle.with_effect_state_ref(|state| state.side).flatten();

        if let Some(side_idx) = side_index {
            let side_id = if side_idx == 0 { "p1" } else { "p2" };
            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "move: Toxic Spikes".into()]);
        }

        // this.effectState.layers = 1;
        battle.with_effect_state(|state| {
            state.layers = Some(1);
        });

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
            .with_effect_state_ref(|state| state.layers.unwrap_or(0))
            .unwrap_or(0);

        if layers >= 2 {
            return EventResult::Boolean(false);
        }

        // this.add('-sidestart', side, 'move: Toxic Spikes');
        let side_index = battle.with_effect_state_ref(|state| state.side).flatten();

        if let Some(side_idx) = side_index {
            let side_id = if side_idx == 0 { "p1" } else { "p2" };
            let side_arg = crate::battle::Arg::Str(side_id);
            battle.add("-sidestart", &[side_arg, "move: Toxic Spikes".into()]);
        }

        // this.effectState.layers++;
        battle.with_effect_state(|state| {
            state.layers = Some(layers + 1);
        });

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
            pokemon_ref.is_grounded(battle, false).unwrap_or(false)
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

            // Get the foe's active Pokemon as the source (pokemon.side.foe.active[0])
            let foe_side = if pokemon.0 == 0 { 1 } else { 0 };
            let foe_source = if !battle.sides[foe_side].pokemon.is_empty()
                && battle.sides[foe_side].active[0].is_some()
            {
                let foe_poke_idx = battle.sides[foe_side].active[0].unwrap();
                Some((foe_side, foe_poke_idx))
            } else {
                None
            };

            // else if (this.effectState.layers >= 2)
            let layers = battle
                .with_effect_state_ref(|state| state.layers.unwrap_or(0))
                .unwrap_or(0);

            if layers >= 2 {
                // pokemon.trySetStatus('tox', pokemon.side.foe.active[0]);
                let _pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                let move_effect = crate::battle::Effect::move_("toxicspikes");
                Pokemon::try_set_status(battle, pokemon_pos, ID::from("tox"), foe_source, Some(&move_effect));
            } else {
                // pokemon.trySetStatus('psn', pokemon.side.foe.active[0]);
                let _pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                let move_effect = crate::battle::Effect::move_("toxicspikes");
                Pokemon::try_set_status(battle, pokemon_pos, ID::from("psn"), foe_source, Some(&move_effect));
            }
        }

        EventResult::Continue
    }
}
