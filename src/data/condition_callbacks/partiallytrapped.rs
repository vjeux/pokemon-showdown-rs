//! Partiallytrapped Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Arg;
use crate::dex_data::ID;
use crate::event::EventResult;

/// durationCallback
/// JavaScript source (data/conditions.ts):
/// partiallytrapped: {
///     durationCallback(target, source) {
///         if (source?.hasItem('gripclaw')) return 8;
///         return this.random(5, 7);
///     }
/// }
pub fn duration_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[PARTIALLYTRAPPED_DURATION_CALLBACK] Called for {:?}", pokemon_pos);

    // In JavaScript: if (source?.hasItem('gripclaw')) return 8;
    // Note: We don't have source info here, so we can't check for Grip Claw
    // For now, just return random 5-6 turns (JavaScript: this.random(5, 7) returns 5 or 6)

    // this.random(5, 7) returns a number from 5 to 6 inclusive
    let duration = battle.prng.random_range(5, 7);

    eprintln!("[PARTIALLYTRAPPED_DURATION_CALLBACK] Duration={}", duration);

    EventResult::Number(duration)
}

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(pokemon, source) {
///     this.add('-activate', pokemon, 'move: ' + this.effectState.sourceEffect, `[of] ${source}`);
///     this.effectState.boundDivisor = source.hasItem('bindingband') ? 6 : 8;
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // Get source and sourceEffect from effectState
    let (source_pos, source_effect_name) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let trap_id = ID::from("partiallytrapped");
        if let Some(state) = pokemon.volatiles.get(&trap_id) {
            let source = state.source;
            // Try to get sourceEffect name from data
            let effect_name = state.data.get("sourceEffect")
                .and_then(|v| v.get("fullname"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            (source, effect_name.to_string())
        } else {
            return EventResult::Continue;
        }
    };

    // this.add('-activate', pokemon, 'move: ' + this.effectState.sourceEffect, `[of] ${source}`);
    if let Some(source) = source_pos {
        let pokemon_ident = {
            let p = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1).unwrap();
            p.get_slot()
        };
        let source_ident = {
            let s = battle.pokemon_at(source.0, source.1);
            s.map(|p| p.get_slot()).unwrap_or_default()
        };

        battle.add("-activate", &[
            pokemon_ident.into(),
            format!("move: {}", source_effect_name).into(),
            format!("[of] {}", source_ident).into(),
        ]);
    }

    // this.effectState.boundDivisor = source.hasItem('bindingband') ? 6 : 8;
    // Check if source has binding band item
    let bound_divisor = if let Some((source_side, source_poke)) = source_pos {
        if let Some(source) = battle.pokemon_at(source_side, source_poke) {
            if source.has_item(battle, &["bindingband"]) {
                6
            } else {
                8
            }
        } else {
            8 // Default if source doesn't exist
        }
    } else {
        8 // Default if no source
    };

    // Store boundDivisor in effectState.data
    {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let trap_id = ID::from("partiallytrapped");
        if let Some(state) = pokemon.volatiles.get_mut(&trap_id) {
            state.data.insert("boundDivisor".to_string(), serde_json::json!(bound_divisor));
        }
    }

    EventResult::Continue
}

/// onResidual
/// JavaScript source (data/conditions.ts):
/// partiallytrapped: {
///     onResidualOrder: 13,
///     onResidual(pokemon) {
///         const source = this.effectState.source;
///         // G-Max Centiferno and G-Max Sandblast continue even after the user leaves the field
///         const gmaxEffect = ['gmaxcentiferno', 'gmaxsandblast'].includes(this.effectState.sourceEffect.id);
///         if (source && (!source.isActive || source.hp <= 0 || !source.activeTurns) && !gmaxEffect) {
///             delete pokemon.volatiles['partiallytrapped'];
///             this.add('-end', pokemon, this.effectState.sourceEffect, '[partiallytrapped]', '[silent]');
///             return;
///         }
///         this.damage(pokemon.baseMaxhp / this.effectState.boundDivisor);
///     }
/// }
pub fn on_residual(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[PARTIALLYTRAPPED_ON_RESIDUAL] Called for {:?}", pokemon_pos);

    // Get source and effect information from volatile state
    let (source_pos, base_maxhp, bound_divisor) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let trap_id = ID::from("partiallytrapped");
        let source = pokemon.volatiles.get(&trap_id)
            .and_then(|state| state.source);

        // JavaScript: this.damage(pokemon.baseMaxhp / this.effectState.boundDivisor);
        // Get boundDivisor from effectState (set in on_start: 6 for Binding Band, 8 otherwise)
        let divisor = pokemon.volatiles.get(&trap_id)
            .and_then(|state| state.data.get("boundDivisor"))
            .and_then(|v| v.as_i64())
            .unwrap_or(8) as i32;

        (source, pokemon.base_maxhp, divisor)
    };

    // JavaScript: if (source && (!source.isActive || source.hp <= 0 || !source.activeTurns) && !gmaxEffect)
    // Check if source Pokemon has fainted or is no longer active
    // G-Max traps (gmaxcentiferno, gmaxsandblast) continue even after source faints
    // TODO: Check for G-Max traps when we implement them
    if let Some((source_side, source_poke)) = source_pos {
        let source_invalid = {
            if let Some(source) = battle.pokemon_at(source_side, source_poke) {
                // Check if source is fainted (hp <= 0) or not active
                source.hp == 0 || source.fainted
            } else {
                // Source doesn't exist anymore
                true
            }
        };

        if source_invalid {
            // Remove the trap volatile without dealing damage
            eprintln!("[PARTIALLYTRAPPED_ON_RESIDUAL] Source Pokemon fainted/inactive, removing trap without damage");

            if let Some(pokemon_mut) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                pokemon_mut.volatiles.remove(&ID::from("partiallytrapped"));
            }

            // JavaScript: this.add('-end', pokemon, this.effectState.sourceEffect, '[partiallytrapped]', '[silent]');
            // TODO: Add battle log message when battle.add() supports this format

            return EventResult::Continue;
        }
    }

    // Calculate damage using boundDivisor from effectState
    // JavaScript: this.damage(pokemon.baseMaxhp / this.effectState.boundDivisor);
    let damage = std::cmp::max(1, base_maxhp / bound_divisor);

    eprintln!("[PARTIALLYTRAPPED_ON_RESIDUAL] Dealing {} damage (maxhp={}, divisor={})",
        damage, base_maxhp, bound_divisor);

    // Deal damage
    battle.damage(damage, Some(pokemon_pos), source_pos, Some(&ID::from("partiallytrapped")), false);

    EventResult::Continue
}

/// onEnd
/// JavaScript source (data/conditions.ts):
/// ```js
/// onEnd(pokemon) {
///     this.add('-end', pokemon, this.effectState.sourceEffect, '[partiallytrapped]');
/// }
/// ```
pub fn on_end(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-end', pokemon, this.effectState.sourceEffect, '[partiallytrapped]');
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    // TODO: Get sourceEffect from effectState to build proper message
    // For now, just add a simple end message
    battle.add("-end", &[Arg::String(pokemon_ident), Arg::Str("[partiallytrapped]")]);

    EventResult::Continue
}

/// onTrapPokemon
/// JavaScript source (data/conditions.ts):
/// ```js
/// onTrapPokemon(pokemon) {
///     const gmaxEffect = ['gmaxcentiferno', 'gmaxsandblast'].includes(this.effectState.sourceEffect.id);
///     if (this.effectState.source?.isActive || gmaxEffect) pokemon.tryTrap();
/// }
/// ```
pub fn on_trap_pokemon(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // const gmaxEffect = ['gmaxcentiferno', 'gmaxsandblast'].includes(this.effectState.sourceEffect.id);
    // TODO: Check for G-Max effects when effectState data is available

    // if (this.effectState.source?.isActive || gmaxEffect) pokemon.tryTrap();
    let trap_id = ID::from("partiallytrapped");
    let source_active = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some(volatile) = pokemon.volatiles.get(&trap_id) {
            if let Some((source_side, source_poke)) = volatile.source {
                if let Some(source) = battle.pokemon_at(source_side, source_poke) {
                    // Check if source is active
                    !source.fainted && source.hp > 0
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    };

    if source_active {
        // pokemon.tryTrap();
        crate::pokemon::Pokemon::try_trap(battle, pokemon_pos, false);
    }

    EventResult::Continue
}

