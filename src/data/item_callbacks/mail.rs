//! Mail Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onTakeItem(item, source) {
///     if (!this.activeMove) return false;
///     if (this.activeMove.id !== 'knockoff' && this.activeMove.id !== 'thief' && this.activeMove.id !== 'covet') return false;
/// }
pub fn on_take_item(battle: &mut Battle, _item_pos: Option<(usize, usize)>, _pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>) -> EventResult {
    // if (!this.activeMove) return false;
    let active_move_id = match &battle.active_move {
        Some(m) => m.id.as_str(),
        None => return EventResult::Boolean(false),
    };

    // if (this.activeMove.id !== 'knockoff' && this.activeMove.id !== 'thief' && this.activeMove.id !== 'covet') return false;
    if active_move_id != "knockoff" && active_move_id != "thief" && active_move_id != "covet" {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}
