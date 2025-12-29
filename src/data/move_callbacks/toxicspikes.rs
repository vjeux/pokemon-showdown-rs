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
    pub fn on_switch_in(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
