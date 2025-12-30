//! Battle Bond Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceAfterFaint(length, target, source, effect) {
///     if (source.bondTriggered) return;
///     if (effect?.effectType !== 'Move') return;
///     if (source.species.id === 'greninjabond' && source.hp && !source.transformed && source.side.foePokemonLeft()) {
///         this.boost({ atk: 1, spa: 1, spe: 1 }, source, source, this.effect);
///         this.add('-activate', source, 'ability: Battle Bond');
///         source.bondTriggered = true;
///     }
/// }
pub fn on_source_after_faint(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyMove(move, attacker) {
///     if (move.id === 'watershuriken' && attacker.species.name === 'Greninja-Ash' &&
///         !attacker.transformed) {
///         move.multihit = 3;
///     }
/// }
pub fn on_modify_move(_battle: &mut Battle, _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

