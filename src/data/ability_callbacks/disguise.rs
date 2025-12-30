//! Disguise Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamage(damage, target, source, effect) {
///     if (effect?.effectType === 'Move' && ['mimikyu', 'mimikyutotem'].includes(target.species.id)) {
///         this.add('-activate', target, 'ability: Disguise');
///         this.effectState.busted = true;
///         return 0;
///     }
/// }
pub fn on_damage(_battle: &mut Battle, _damage: i32, _target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onCriticalHit(target, source, move) {
///     if (!target) return;
///     if (!['mimikyu', 'mimikyutotem'].includes(target.species.id)) {
///         return;
///     }
///     const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
///     if (hitSub) return;
/// 
///     if (!target.runImmunity(move)) return;
///     return false;
/// }
pub fn on_critical_hit(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onEffectiveness(typeMod, target, type, move) {
///     if (!target || move.category === 'Status') return;
///     if (!['mimikyu', 'mimikyutotem'].includes(target.species.id)) {
///         return;
///     }
/// 
///     const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
///     if (hitSub) return;
/// 
///     if (!target.runImmunity(move)) return;
///     return 0;
/// }
pub fn on_effectiveness(_battle: &mut Battle, _damage: i32, _target_pos: (usize, usize), _type_str: &str, _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onUpdate(pokemon) {
///     if (['mimikyu', 'mimikyutotem'].includes(pokemon.species.id) && this.effectState.busted) {
///         const speciesid = pokemon.species.id === 'mimikyutotem' ? 'Mimikyu-Busted-Totem' : 'Mimikyu-Busted';
///         pokemon.formeChange(speciesid, this.effect, true);
///         this.damage(pokemon.baseMaxhp / 8, pokemon, pokemon, this.dex.species.get(speciesid));
///     }
/// }
pub fn on_update(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

