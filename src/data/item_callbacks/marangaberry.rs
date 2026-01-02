//! Maranga Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onAfterMoveSecondary(target, source, move) {
///     if (move.category === 'Special') {
///         target.eatItem();
///     }
/// }
pub fn on_after_move_secondary(battle: &mut Battle, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // if (move.category === 'Special') {
    //     target.eatItem();
    // }

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Check if move category is Special
    let is_special = battle.active_move.as_ref()
        .map(|m| m.category == "Special")
        .unwrap_or(false);

    if !is_special {
        return EventResult::Continue;
    }

    // target.eatItem();
    Pokemon::eat_item(battle, target_pos, false, None, None);

    EventResult::Continue
}

/// onEat(pokemon) {
///     this.boost({ spd: 1 });
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // this.boost({ spd: 1 });
    battle.boost(&[("spd", 1)], pokemon_pos, None, None, false, false);
    EventResult::Continue
}
