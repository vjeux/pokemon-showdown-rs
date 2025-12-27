//! Teatime Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

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
pub fn on_hit_field(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

