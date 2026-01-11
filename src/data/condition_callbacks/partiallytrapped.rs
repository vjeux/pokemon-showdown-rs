//! Partiallytrapped Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Arg;
use crate::battle::Effect;
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
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    eprintln!("[PARTIALLYTRAPPED_DURATION_CALLBACK] Called for source_pos={:?}", source_pos);

    // if (source?.hasItem('gripclaw')) return 8;
    let has_grip_claw = if let Some(pos) = source_pos {
        let pokemon = match battle.pokemon_at(pos.0, pos.1) {
            Some(p) => p,
            None => return EventResult::Number(battle.prng.random_range(5, 7)),
        };
        pokemon.has_item(battle, &["gripclaw"])
    } else {
        false
    };

    if has_grip_claw {
        eprintln!("[PARTIALLYTRAPPED_DURATION_CALLBACK] Source has Grip Claw, duration=8");
        EventResult::Number(8)
    } else {
        // this.random(5, 7) returns a number from 5 to 6 inclusive
        let duration = battle.prng.random_range(5, 7);
        eprintln!("[PARTIALLYTRAPPED_DURATION_CALLBACK] Duration={}", duration);
        EventResult::Number(duration)
    }
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
    _source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    // Get source and sourceEffect from effectState using with_effect_state_ref
    // JavaScript: this.effectState.source, this.effectState.sourceEffect
    let (source_pos, source_effect_name) = match battle.with_effect_state_ref(|state| {
        let source = state.source;
        let effect_name = state.source_effect
            .as_ref()
            .map(|id| id.as_str().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        (source, effect_name)
    }) {
        Some(result) => result,
        None => return EventResult::Continue,
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

    // Store boundDivisor in effectState
    // JavaScript: this.effectState.boundDivisor = ...
    battle.with_effect_state(|state| {
        state.bound_divisor = Some(bound_divisor);
    });

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
    _source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    eprintln!("[PARTIALLYTRAPPED_ON_RESIDUAL] Called for {:?}", pokemon_pos);

    // Get source, effect information, and boundDivisor from volatile state using with_effect_state_ref
    // JavaScript: this.effectState.source, this.effectState.sourceEffect, this.effectState.boundDivisor
    let (source_pos, bound_divisor, is_gmax_effect, source_effect_name) = match battle.with_effect_state_ref(|state| {
        let source = state.source;

        // JavaScript: this.damage(pokemon.baseMaxhp / this.effectState.boundDivisor);
        let divisor = state.bound_divisor.unwrap_or(8);

        // JavaScript: const gmaxEffect = ['gmaxcentiferno', 'gmaxsandblast'].includes(this.effectState.sourceEffect.id);
        let gmax = state.source_effect.as_ref()
            .map(|id| {
                let id_str = id.as_str();
                let is_gmax = id_str == "gmaxcentiferno" || id_str == "gmaxsandblast";
                eprintln!("[PARTIALLYTRAPPED_DEBUG] source_effect='{}', is_gmax={}", id_str, is_gmax);
                is_gmax
            })
            .unwrap_or(false);

        let effect_name = state.source_effect
            .as_ref()
            .map(|id| id.as_str().to_string())
            .unwrap_or_else(|| "[partiallytrapped]".to_string());

        (source, divisor, gmax, effect_name)
    }) {
        Some(result) => result,
        None => return EventResult::Continue,
    };

    // Get base_maxhp from pokemon
    let base_maxhp = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.base_maxhp
    };

    // JavaScript: if (source && (!source.isActive || source.hp <= 0 || !source.activeTurns) && !gmaxEffect)
    // Check if source Pokemon has fainted, is no longer active, or has no active turns
    // G-Max traps (gmaxcentiferno, gmaxsandblast) continue even after source faints
    if let Some((source_side, source_poke)) = source_pos {
        let source_invalid = {
            if let Some(source) = battle.pokemon_at(source_side, source_poke) {
                // Check if source is fainted (hp <= 0), not active, or has no active turns
                // JS: !source.isActive || source.hp <= 0 || !source.activeTurns
                !source.is_active || source.hp == 0 || source.fainted || source.active_turns == 0
            } else {
                // Source doesn't exist anymore
                true
            }
        };

        // Only remove trap if source is invalid AND it's not a G-Max effect
        if source_invalid && !is_gmax_effect {
            // Remove the trap volatile without dealing damage
            eprintln!("[PARTIALLYTRAPPED_ON_RESIDUAL] Source Pokemon fainted/inactive, removing trap without damage");

            // Remove the volatile
            if let Some(pokemon_mut) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                pokemon_mut.volatiles.remove(&ID::from("partiallytrapped"));
            }

            // JavaScript: this.add('-end', pokemon, this.effectState.sourceEffect, '[partiallytrapped]', '[silent]');
            let pokemon_ident = {
                let p = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1).unwrap();
                p.get_slot()
            };
            battle.add("-end", &[
                Arg::String(pokemon_ident),
                Arg::String(source_effect_name),
                Arg::Str("[partiallytrapped]"),
                Arg::Str("[silent]")
            ]);

            return EventResult::Continue;
        }
    }

    // Calculate damage using boundDivisor from effectState
    // JavaScript: this.damage(pokemon.baseMaxhp / this.effectState.boundDivisor);
    let damage = std::cmp::max(1, base_maxhp / bound_divisor);

    eprintln!("[PARTIALLYTRAPPED_ON_RESIDUAL] Dealing {} damage (maxhp={}, divisor={})",
        damage, base_maxhp, bound_divisor);

    // Deal damage
    battle.damage(damage, Some(pokemon_pos), source_pos, Some(&Effect::condition("partiallytrapped")), false);

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

    // Get sourceEffect from effectState using with_effect_state_ref
    // JavaScript: this.effectState.sourceEffect
    let source_effect_name = battle.with_effect_state_ref(|state| {
        state.source_effect
            .as_ref()
            .map(|id| id.as_str().to_string())
            .unwrap_or_else(|| "[partiallytrapped]".to_string())
    }).unwrap_or_else(|| "[partiallytrapped]".to_string());

    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-end", &[
        Arg::String(pokemon_ident),
        Arg::String(source_effect_name),
        Arg::Str("[partiallytrapped]")
    ]);

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
    // JavaScript: const gmaxEffect = ['gmaxcentiferno', 'gmaxsandblast'].includes(this.effectState.sourceEffect.id);
    // JavaScript: if (this.effectState.source?.isActive || gmaxEffect) pokemon.tryTrap();

    // Get source and is_gmax_effect from effectState using with_effect_state_ref
    // JavaScript: this.effectState.source, this.effectState.sourceEffect
    let (source_pos, is_gmax_effect) = match battle.with_effect_state_ref(|state| {
        let source = state.source;

        // Check for G-Max effects (gmaxcentiferno, gmaxsandblast)
        let gmax = state.source_effect.as_ref()
            .map(|id| {
                let id_str = id.as_str();
                id_str == "gmaxcentiferno" || id_str == "gmaxsandblast"
            })
            .unwrap_or(false);

        (source, gmax)
    }) {
        Some(result) => result,
        None => return EventResult::Continue,
    };

    // Check if source is active
    // JavaScript: this.effectState.source?.isActive
    let source_active = if let Some((source_side, source_poke)) = source_pos {
        if let Some(source) = battle.pokemon_at(source_side, source_poke) {
            // Check if source is active (the is_active field matches JS's isActive property)
            source.is_active
        } else {
            false
        }
    } else {
        false
    };

    // Trap pokemon if source is active OR it's a G-Max effect
    if source_active || is_gmax_effect {
        // pokemon.tryTrap();
        crate::pokemon::Pokemon::try_trap(battle, pokemon_pos, false);
    }

    EventResult::Continue
}

