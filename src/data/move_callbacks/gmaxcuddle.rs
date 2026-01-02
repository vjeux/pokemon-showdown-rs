//! G-Max Cuddle Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onHit(source)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(source) {
/// 				for (const pokemon of source.foes()) {
/// 					pokemon.addVolatile('attract');
/// 				}
/// 			},
///
/// 		}
/// ```
pub fn on_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // for (const pokemon of source.foes()) {
    //     pokemon.addVolatile('attract');
    // }
    let foe_positions = {
        let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.foes(battle, false)
    };

    for foe_pos in foe_positions {
        Pokemon::add_volatile(battle, foe_pos, ID::from("attract"), Some(source_pos));
    }

    EventResult::Continue
}

