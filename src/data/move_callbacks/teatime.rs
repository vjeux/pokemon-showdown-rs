//! Teatime Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHitField(target, source, move) {
///     const targets: Pokemon[] = [];
///     for (const pokemon of this.getAllActive()) {
///         if (this.runEvent('Invulnerability', pokemon, source, move) === false) {
///             this.add('-miss', source, pokemon);
///         } else if (this.runEvent('TryHit', pokemon, source, move) && pokemon.getItem().isBerry) {
///             targets.push(pokemon);
///         }
///     }
///     this.add('-fieldactivate', 'move: Teatime');
///     if (!targets.length) {
///         this.add('-fail', source, 'move: Teatime');
///         this.attrLastMove('[still]');
///         return this.NOT_FAIL;
///     }
///     for (const pokemon of targets) {
///         pokemon.eatItem(true);
///     }
/// }
pub fn on_hit_field(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

