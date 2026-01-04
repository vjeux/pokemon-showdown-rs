//! Partiallytrapped Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
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
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// partiallytrapped: {
///     onStart(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[PARTIALLYTRAPPED_ON_START] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
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
    let (source_pos, base_maxhp, gen) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let trap_id = ID::from("partiallytrapped");
        let source = pokemon.volatiles.get(&trap_id)
            .and_then(|state| state.source);

        (source, pokemon.base_maxhp, battle.gen)
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

    // Calculate damage: Gen 1 = 1/16 max HP, Gen 2+ = 1/8 max HP
    let divisor = if gen >= 2 { 8 } else { 16 };
    let damage = std::cmp::max(1, base_maxhp / divisor);

    eprintln!("[PARTIALLYTRAPPED_ON_RESIDUAL] Dealing {} damage (maxhp={}, divisor={})",
        damage, base_maxhp, divisor);

    // Deal damage
    // JavaScript: this.damage(pokemon.baseMaxhp / this.effectState.boundDivisor);
    // In Rust: battle.damage(damage, target, source, effect, instafaint)
    use crate::dex_data::ID;
    battle.damage(damage, Some(pokemon_pos), source_pos, Some(&ID::from("partiallytrapped")), false);

    EventResult::Continue
}

/// onEnd
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// partiallytrapped: {
///     onEnd(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_end(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[PARTIALLYTRAPPED_ON_END] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

/// onTrapPokemon
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// partiallytrapped: {
///     onTrapPokemon(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_trap_pokemon(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[PARTIALLYTRAPPED_ON_TRAP_POKEMON] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

