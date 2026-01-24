//! Spirit Shackle Move
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
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (source.isActive) target.addVolatile('trapped', source, move, 'trapper');

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Check if source is active
    let source_is_active = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.is_active
    };

    if source_is_active {
        // target.addVolatile('trapped', source, move, 'trapper');
        // JavaScript: target.addVolatile('trapped', source, move, 'trapper')
        // ✅ NOW PASSING: source_pos = Some(source), source_effect = Some("spiritshackle"), linked_status = Some("trapper")
        let move_effect = battle.make_move_effect(&ID::from("spiritshackle"));
        Pokemon::add_volatile(
            battle,
            target_pos,
            ID::from("trapped"),
            Some(source),
            Some(&move_effect),
            Some(ID::from("trapper")),
            None,
        );
    }

    EventResult::Continue
}
