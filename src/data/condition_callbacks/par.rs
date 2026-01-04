//! Par Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onBeforeMovePriority
/// JavaScript source (data/conditions.ts):
/// par: {
///     onBeforeMovePriority: 1,
/// }
pub fn on_before_move_priority(
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Number(1)
}

/// onBeforeMove
/// JavaScript source (data/conditions.ts):
/// par: {
///     onBeforeMove(pokemon) {
///         if (this.randomChance(1, 4)) {
///             this.add('cant', pokemon, 'par');
///             return false;
///         }
///     },
/// }
pub fn on_before_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[PAR] onBeforeMove - Checking paralysis 25% chance to prevent move");

    // 25% chance to be fully paralyzed
    if battle.random_chance(1, 4) {
        eprintln!("[PAR] Pokemon is fully paralyzed!");
        // TODO: Add battle message - battle.add("cant", pokemon, "par")
        return EventResult::Boolean(false);
    }

    eprintln!("[PAR] Pokemon overcomes paralysis");
    EventResult::Continue
}

/// onStart
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// par: {
///     onStart(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[PAR_ON_START] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

