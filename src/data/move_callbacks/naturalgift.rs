//! Natural Gift Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     if (pokemon.ignoringItem()) return;
///     const item = pokemon.getItem();
///     if (!item.naturalGift) return;
///     move.type = item.naturalGift.type;
/// }
pub fn on_modify_type(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
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
pub fn on_prepare_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

