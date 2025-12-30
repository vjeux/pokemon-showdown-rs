//! Illusion Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBeforeSwitchIn(pokemon) {
///     pokemon.illusion = null;
///     // yes, you can Illusion an active pokemon but only if it's to your right
///     for (let i = pokemon.side.pokemon.length - 1; i > pokemon.position; i--) {
///         const possibleTarget = pokemon.side.pokemon[i];
///         if (!possibleTarget.fainted) {
///             // If Ogerpon is in the last slot while the Illusion Pokemon is Terastallized
///             // Illusion will not disguise as anything
///             if (!pokemon.terastallized || !['Ogerpon', 'Terapagos'].includes(possibleTarget.species.baseSpecies)) {
///                 pokemon.illusion = possibleTarget;
///             }
///             break;
///         }
///     }
/// }
pub fn on_before_switch_in(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onDamagingHit(damage, target, source, move) {
///     if (target.illusion) {
///         this.singleEvent('End', this.dex.abilities.get('Illusion'), target.abilityState, target, source, move);
///     }
/// }
pub fn on_damaging_hit(_battle: &mut Battle, _damage: i32, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onEnd(pokemon) {
///     if (pokemon.illusion) {
///         this.debug('illusion cleared');
///         pokemon.illusion = null;
///         const details = pokemon.getUpdatedDetails();
///         this.add('replace', pokemon, details);
///         this.add('-end', pokemon, 'Illusion');
///         if (this.ruleTable.has('illusionlevelmod')) {
///             this.hint("Illusion Level Mod is active, so this Pok\u00e9mon's true level was hidden.", true);
///         }
///     }
/// }
pub fn on_end(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onFaint(pokemon) {
///     pokemon.illusion = null;
/// }
pub fn on_faint(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

