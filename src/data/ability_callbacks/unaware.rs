//! Unaware Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAnyModifyBoost(boosts, pokemon) {
///     const unawareUser = this.effectState.target;
///     if (unawareUser === pokemon) return;
///     if (unawareUser === this.activePokemon && pokemon === this.activeTarget) {
///         boosts['def'] = 0;
///         boosts['spd'] = 0;
///         boosts['evasion'] = 0;
///     }
///     if (pokemon === this.activePokemon && unawareUser === this.activeTarget) {
///         boosts['atk'] = 0;
///         boosts['def'] = 0;
///         boosts['spa'] = 0;
///         boosts['accuracy'] = 0;
///     }
/// }
pub fn on_any_modify_boost(battle: &mut Battle, _boosts: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // const unawareUser = this.effectState.target;
    let unaware_user = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (unawareUser === pokemon) return;
    if unaware_user == pokemon_pos {
        return EventResult::Continue;
    }

    // Get active pokemon and active target from battle
    let active_pokemon = battle.active_pokemon;
    let active_target = battle.active_target;

    // Access boosts from current event (similar to Contrary)
    if let Some(ref mut event) = battle.current_event {
        if let Some(ref mut boosts) = event.relay_var_boost {
            // if (unawareUser === this.activePokemon && pokemon === this.activeTarget)
            if Some(unaware_user) == active_pokemon && Some(pokemon_pos) == active_target {
                // boosts['def'] = 0; boosts['spd'] = 0; boosts['evasion'] = 0;
                boosts.def = 0;
                boosts.spd = 0;
                boosts.evasion = 0;
            }

            // if (pokemon === this.activePokemon && unawareUser === this.activeTarget)
            if Some(pokemon_pos) == active_pokemon && Some(unaware_user) == active_target {
                // boosts['atk'] = 0; boosts['def'] = 0; boosts['spa'] = 0; boosts['accuracy'] = 0;
                boosts.atk = 0;
                boosts.def = 0;
                boosts.spa = 0;
                boosts.accuracy = 0;
            }
        }
    }

    EventResult::Continue
}

