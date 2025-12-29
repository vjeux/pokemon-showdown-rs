//! Ice Face Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (this.field.isWeather(['hail', 'snowscape']) && pokemon.species.id === 'eiscuenoice') {
///         this.add('-activate', pokemon, 'ability: Ice Face');
///         this.effectState.busted = false;
///         pokemon.formeChange('Eiscue', this.effect, true);
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onDamage(damage, target, source, effect) {
///     if (effect?.effectType === 'Move' && effect.category === 'Physical' && target.species.id === 'eiscue') {
///         this.add('-activate', target, 'ability: Ice Face');
///         this.effectState.busted = true;
///         return 0;
///     }
/// }
pub fn on_damage(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onCriticalHit(target, type, move) {
///     if (!target) return;
///     if (move.category !== 'Physical' || target.species.id !== 'eiscue') return;
///     if (target.volatiles['substitute'] && !(move.flags['bypasssub'] || move.infiltrates)) return;
///     if (!target.runImmunity(move)) return;
///     return false;
/// }
pub fn on_critical_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onEffectiveness(typeMod, target, type, move) {
///     if (!target) return;
///     if (move.category !== 'Physical' || target.species.id !== 'eiscue') return;
/// 
///     const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
///     if (hitSub) return;
/// 
///     if (!target.runImmunity(move)) return;
///     return 0;
/// }
pub fn on_effectiveness(battle: &mut Battle, pokemon_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onUpdate(pokemon) {
///     if (pokemon.species.id === 'eiscue' && this.effectState.busted) {
///         pokemon.formeChange('Eiscue-Noice', this.effect, true);
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onWeatherChange(pokemon, source, sourceEffect) {
///     // snow/hail resuming because Cloud Nine/Air Lock ended does not trigger Ice Face
///     if ((sourceEffect as Ability)?.suppressWeather) return;
///     if (!pokemon.hp) return;
///     if (this.field.isWeather(['hail', 'snowscape']) && pokemon.species.id === 'eiscuenoice') {
///         this.add('-activate', pokemon, 'ability: Ice Face');
///         this.effectState.busted = false;
///         pokemon.formeChange('Eiscue', this.effect, true);
///     }
/// }
pub fn on_weather_change(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

