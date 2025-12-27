//! Swallow Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTry(source, target, move) {
///     if (move.sourceEffect === 'snatch') return;
///     return !!source.volatiles['stockpile'];
/// }
pub fn on_try(battle: &mut Battle, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onHit(pokemon) {
///     const layers = pokemon.volatiles['stockpile']?.layers || 1;
///     const healAmount = [0.25, 0.5, 1];
///     const success = !!this.heal(this.modify(pokemon.maxhp, healAmount[layers - 1]));
///     if (!success) this.add('-fail', pokemon, 'heal');
///     pokemon.removeVolatile('stockpile');
///     return success || this.NOT_FAIL;
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize)) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

