//! Skill Swap Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source) {
///     const targetAbility = target.getAbility();
///     const sourceAbility = source.getAbility();
///     if (sourceAbility.flags['failskillswap'] || targetAbility.flags['failskillswap'] || target.volatiles['dynamax']) {
///         return false;
///     }
///     const sourceCanBeSet = this.runEvent('SetAbility', source, source, this.effect, targetAbility);
///     if (!sourceCanBeSet) return sourceCanBeSet;
///     const targetCanBeSet = this.runEvent('SetAbility', target, source, this.effect, sourceAbility);
///     if (!targetCanBeSet) return targetCanBeSet;
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize), move_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onHit(target, source, move) {
///     const targetAbility = target.getAbility();
///     const sourceAbility = source.getAbility();
///     if (target.isAlly(source)) {
///         this.add('-activate', source, 'move: Skill Swap', '', '', `[of] ${target}`);
///     } else {
///         this.add('-activate', source, 'move: Skill Swap', targetAbility, sourceAbility, `[of] ${target}`);
///     }
///     this.singleEvent('End', sourceAbility, source.abilityState, source);
///     this.singleEvent('End', targetAbility, target.abilityState, target);
///     source.ability = targetAbility.id;
///     target.ability = sourceAbility.id;
///     source.abilityState = this.initEffectState({ id: this.toID(source.ability), target: source });
///     target.abilityState = this.initEffectState({ id: this.toID(target.ability), target });
///     source.volatileStaleness = undefined;
///     if (!target.isAlly(source)) target.volatileStaleness = 'external';
///     this.singleEvent('Start', targetAbility, source.abilityState, source);
///     this.singleEvent('Start', sourceAbility, target.abilityState, target);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

