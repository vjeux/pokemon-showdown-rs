//! Water Gem Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onSourceTryPrimaryHit(target, source, move) {
///     if (target === source || move.category === 'Status' || move.flags['pledgecombo']) return;
///     if (move.type === 'Water' && source.useItem()) {
///         source.addVolatile('gem');
///     }
/// }
pub fn on_source_try_primary_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (target === source || move.category === 'Status' || move.flags['pledgecombo']) return;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // target === source
    if target == source {
        return EventResult::Continue;
    }

    // move.category === 'Status' || move.flags['pledgecombo']
    let (is_status, is_pledgecombo) = match &battle.active_move {
        Some(active_move) => (
            active_move.category == "Status",
            active_move.flags.pledgecombo,
        ),
        None => return EventResult::Continue,
    };

    if is_status || is_pledgecombo {
        return EventResult::Continue;
    }

    // if (move.type === 'Water' && source.useItem())
    let is_water = match &battle.active_move {
        Some(active_move) => active_move.move_type == "Water",
        None => return EventResult::Continue,
    };

    if is_water {
        let used_item = {
            let _source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            Pokemon::use_item(battle, source, None, None).is_some()
        };

        if used_item {
            // source.addVolatile('gem');
            Pokemon::add_volatile(battle, source, ID::new("gem"), None, None, None, None);
        }
    }

    EventResult::Continue
}
