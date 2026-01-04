//! Partiallytrapped Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// durationCallback
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// partiallytrapped: {
///     durationCallback(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn duration_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[PARTIALLYTRAPPED_DURATION_CALLBACK] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
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
///     onResidual(pokemon) {
///         const source = this.effectState.source;
///         // Gen 1 damage is 1/16, Gen 2+ damage is 1/8
///         const damage = this.clampIntRange(pokemon.baseMaxhp / (this.gen >= 2 ? 8 : 16), 1);
///         this.damage(damage, pokemon, source);
///     }
/// }
pub fn on_residual(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[PARTIALLYTRAPPED_ON_RESIDUAL] Called for {:?}", pokemon_pos);

    // Get pokemon and extract needed data
    let (base_maxhp, gen) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.base_maxhp, battle.gen)
    };

    // Calculate damage: Gen 1 = 1/16 max HP, Gen 2+ = 1/8 max HP
    let divisor = if gen >= 2 { 8 } else { 16 };
    let damage = std::cmp::max(1, base_maxhp / divisor);

    eprintln!("[PARTIALLYTRAPPED_ON_RESIDUAL] Dealing {} damage (maxhp={}, divisor={})",
        damage, base_maxhp, divisor);

    // Deal damage
    // JavaScript: this.damage(damage, pokemon, source);
    // In Rust: battle.damage(damage, target, source, effect, instafaint)
    use crate::dex_data::ID;
    battle.damage(damage, Some(pokemon_pos), None, Some(&ID::from("partiallytrapped")), false);

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

