//! Ivy Cudgel Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onPrepareHit(target, source, move) {
///     if (move.type !== "Grass") {
///         this.attrLastMove('[anim] Ivy Cudgel ' + move.type);
///     }
/// }
pub fn on_prepare_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyType(move, pokemon) {
///     switch (pokemon.species.name) {
///     case 'Ogerpon-Wellspring': case 'Ogerpon-Wellspring-Tera':
///         move.type = 'Water';
///         break;
///     case 'Ogerpon-Hearthflame': case 'Ogerpon-Hearthflame-Tera':
///         move.type = 'Fire';
///         break;
///     case 'Ogerpon-Cornerstone': case 'Ogerpon-Cornerstone-Tera':
///         move.type = 'Rock';
///         break;
///     }
/// }
pub fn on_modify_type(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

