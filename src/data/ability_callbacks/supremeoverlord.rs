//! Supreme Overlord Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (pokemon.side.totalFainted) {
///         this.add('-activate', pokemon, 'ability: Supreme Overlord');
///         const fallen = Math.min(pokemon.side.totalFainted, 5);
///         this.add('-start', pokemon, `fallen${fallen}`, '[silent]');
///         this.effectState.fallen = fallen;
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    use crate::battle::Arg;

    // if (pokemon.side.totalFainted)
    let total_fainted = {
        let side = &battle.sides[pokemon_pos.0];
        side.total_fainted
    };

    if total_fainted == 0 {
        return EventResult::Continue;
    }

    // this.add('-activate', pokemon, 'ability: Supreme Overlord');
    let pokemon_id = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-activate", &[
        Arg::String(pokemon_id.clone()),
        Arg::Str("ability: Supreme Overlord"),
    ]);

    // const fallen = Math.min(pokemon.side.totalFainted, 5);
    let fallen = std::cmp::min(total_fainted, 5);

    // this.add('-start', pokemon, `fallen${fallen}`, '[silent]');
    battle.add("-start", &[
        Arg::String(pokemon_id),
        Arg::String(format!("fallen{}", fallen)),
        Arg::Str("[silent]"),
    ]);

    // this.effectState.fallen = fallen;
    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    pokemon_mut.ability_state.fallen = Some(fallen as i32);

    EventResult::Continue
}

/// onEnd(pokemon) {
///     this.add('-end', pokemon, `fallen${this.effectState.fallen}`, '[silent]');
/// }
pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;

    // Get fallen from ability state
    let fallen = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.fallen.unwrap_or(0)
    };

    if fallen == 0 {
        return EventResult::Continue;
    }

    // this.add('-end', pokemon, `fallen${this.effectState.fallen}`, '[silent]');
    let pokemon_id = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-end", &[
        Arg::String(pokemon_id),
        Arg::String(format!("fallen{}", fallen)),
        Arg::Str("[silent]"),
    ]);

    EventResult::Continue
}

/// onBasePower(basePower, attacker, defender, move) {
///     if (this.effectState.fallen) {
///         const powMod = [4096, 4506, 4915, 5325, 5734, 6144];
///         this.debug(`Supreme Overlord boost: ${powMod[this.effectState.fallen]}/4096`);
///         return this.chainModify([powMod[this.effectState.fallen], 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (this.effectState.fallen)
    let fallen = {
        let pokemon = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.fallen.unwrap_or(0) as usize
    };

    if fallen == 0 {
        return EventResult::Continue;
    }

    // const powMod = [4096, 4506, 4915, 5325, 5734, 6144];
    let pow_mod = [4096, 4506, 4915, 5325, 5734, 6144];

    // Bounds check: fallen should be 1-5 (index 1-5 in array)
    if fallen > 5 {
        return EventResult::Continue;
    }

    // this.debug(`Supreme Overlord boost: ${powMod[this.effectState.fallen]}/4096`);
    // return this.chainModify([powMod[this.effectState.fallen], 4096]);
    { battle.chain_modify_fraction(pow_mod[fallen], 4096); EventResult::Continue }
}

