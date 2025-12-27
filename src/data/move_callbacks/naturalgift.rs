//! Natural Gift Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onModifyType(move, pokemon) {
///     if (pokemon.ignoringItem()) return;
///     const item = pokemon.getItem();
///     if (!item.naturalGift) return;
///     move.type = item.naturalGift.type;
/// }
pub fn on_modify_type(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onPrepareHit(target, pokemon, move) {
///     if (pokemon.ignoringItem()) return false;
///     const item = pokemon.getItem();
///     if (!item.naturalGift) return false;
///     move.basePower = item.naturalGift.basePower;
///     this.debug(`BP: ${move.basePower}`);
///     pokemon.setItem('');
///     pokemon.lastItem = item.id;
///     pokemon.usedItemThisTurn = true;
///     this.runEvent('AfterUseItem', pokemon, null, null, item);
/// }
pub fn on_prepare_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

