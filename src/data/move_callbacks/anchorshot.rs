//! Anchor Shot Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onHit(target, source, move)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(target, source, move) {
/// 				if (source.isActive) target.addVolatile('trapped', source, move, 'trapper');
/// 			},
///
/// 		}
/// ```
pub fn on_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (source.isActive) target.addVolatile('trapped', source, move, 'trapper');

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Check if source is active
    let source_is_active = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.is_active
    };

    if source_is_active {
        Pokemon::add_volatile(battle, target, ID::from("trapped"), Some(source_pos), None);
    }

    EventResult::Continue
}
