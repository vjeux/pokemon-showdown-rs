//! Oblivious Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     if (pokemon.volatiles['attract']) {
///         this.add('-activate', pokemon, 'ability: Oblivious');
///         pokemon.removeVolatile('attract');
///         this.add('-end', pokemon, 'move: Attract', '[from] ability: Oblivious');
///     }
///     if (pokemon.volatiles['taunt']) {
///         this.add('-activate', pokemon, 'ability: Oblivious');
///         pokemon.removeVolatile('taunt');
///         // Taunt's volatile already sends the -end message when removed
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onImmunity(type, pokemon) {
///     if (type === 'attract') return false;
/// }
pub fn on_immunity(battle: &mut Battle, type_or_status: &str, pokemon_pos: (usize, usize)) -> EventResult {
    if type_or_status == "attract" {
        return EventResult::Boolean(false);
    }
    EventResult::Continue
}

/// onTryHit(pokemon, target, move) {
///     if (move.id === 'attract' || move.id === 'captivate' || move.id === 'taunt') {
///         this.add('-immune', pokemon, '[from] ability: Oblivious');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onTryBoost(boost, target, source, effect) {
///     if (effect.name === 'Intimidate' && boost.atk) {
///         delete boost.atk;
///         this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Oblivious', `[of] ${target}`);
///     }
/// }
pub fn on_try_boost(battle: &mut Battle, boost: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

