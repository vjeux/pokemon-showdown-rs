//! Maranga Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;
use crate::Pokemon;

/// onAfterMoveSecondary(target, source, move) {
///     if (move.category === 'Special') {
///         target.eatItem();
///     }
/// }
pub fn on_after_move_secondary(battle: &mut Battle, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (move.category === 'Special') {
    //     target.eatItem();
    // }

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Check if move category is Special
    if active_move_ref.category != "Special" {
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
