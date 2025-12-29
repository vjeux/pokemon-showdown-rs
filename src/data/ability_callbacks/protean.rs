//! Protean Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onPrepareHit(source, target, move) {
///     if (this.effectState.protean) return;
///     if (move.hasBounced || move.flags['futuremove'] || move.sourceEffect === 'snatch' || move.callsMove) return;
///     const type = move.type;
///     if (type && type !== '???' && source.getTypes().join() !== type) {
///         if (!source.setType(type)) return;
///         this.effectState.protean = true;
///         this.add('-start', source, 'typechange', type, '[from] ability: Protean');
///     }
/// }
pub fn on_prepare_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

