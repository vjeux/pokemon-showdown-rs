//! Fury Cutter Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// basePowerCallback(pokemon, target, move) {
///     if (!pokemon.volatiles['furycutter'] || move.hit === 1) {
///         pokemon.addVolatile('furycutter');
///     }
///     const bp = this.clampIntRange(move.basePower * pokemon.volatiles['furycutter'].multiplier, 1, 160);
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;
    let furycutter_id = ID::from("furycutter");

    // if (!pokemon.volatiles['furycutter'] || move.hit === 1) {
    //     pokemon.addVolatile('furycutter');
    // }
    let (has_furycutter, move_hit) = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let has_volatile = pokemon_ref.volatiles.contains_key(&furycutter_id);
        let hit = battle
            .active_move
            .as_ref()
            .map(|m| m.hit)
            .unwrap_or(0);

        (has_volatile, hit)
    };

    if !has_furycutter || move_hit == 1 {
        Pokemon::add_volatile(battle, pokemon, furycutter_id.clone(), None, None, None, None);
    }

    // const bp = this.clampIntRange(move.basePower * pokemon.volatiles['furycutter'].multiplier, 1, 160);
    let (base_power, multiplier) = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_power = battle
            .active_move
            .as_ref()
            .map(|m| m.base_power)
            .unwrap_or(0);

        let multiplier = pokemon_ref
            .volatiles
            .get(&furycutter_id)
            .and_then(|v| v.borrow().multiplier)
            .unwrap_or(1);

        (base_power, multiplier)
    };

    let bp = (base_power * multiplier as i32).max(1).min(160);

    // this.debug(`BP: ${bp}`);
    battle.debug(&format!("BP: {}", bp));

    // return bp;
    EventResult::Number(bp)
}

pub mod condition {
    use super::*;

    /// onStart() {
    ///     this.effectState.multiplier = 1;
    /// }
    pub fn on_start(
        battle: &mut Battle,
        _pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        // this.effectState.multiplier = 1;
        battle.with_effect_state(|state| {
            state.multiplier = Some(1);
        });

        EventResult::Continue
    }

    /// onRestart() {
    ///     if (this.effectState.multiplier < 4) {
    ///         this.effectState.multiplier <<= 1;
    ///     }
    ///     this.effectState.duration = 2;
    /// }
    pub fn on_restart(battle: &mut Battle) -> EventResult {
        // if (this.effectState.multiplier < 4) {
        //     this.effectState.multiplier <<= 1;
        // }
        // this.effectState.duration = 2;
        battle.with_effect_state(|state| {
            let current_multiplier = state.multiplier.unwrap_or(1);

            if current_multiplier < 4 {
                // this.effectState.multiplier <<= 1;
                let new_multiplier = current_multiplier << 1; // Left shift by 1 = multiply by 2
                state.multiplier = Some(new_multiplier);
            }

            // this.effectState.duration = 2;
            state.duration = Some(2);
        });

        EventResult::Continue
    }
}
