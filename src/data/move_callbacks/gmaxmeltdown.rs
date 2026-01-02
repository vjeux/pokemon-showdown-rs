//! G-Max Meltdown Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onHit(source, target, effect)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(source, target, effect) {
/// 				for (const pokemon of source.foes()) {
/// 					if (!pokemon.volatiles['dynamax']) pokemon.addVolatile('torment', source, effect);
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
    //     if (!pokemon.volatiles['dynamax']) pokemon.addVolatile('torment', source, effect);
    // }
    let source_side = source_pos.0;
    let foe_side = 1 - source_side;

    let foe_positions: Vec<(usize, usize)> = battle
        .get_all_active(false)
        .into_iter()
        .filter(|(side_idx, _)| *side_idx == foe_side)
        .collect();

    for foe_pos in foe_positions {
        let has_dynamax = {
            let foe = match battle.pokemon_at(foe_pos.0, foe_pos.1) {
                Some(p) => p,
                None => continue,
            };
            foe.has_volatile(&ID::from("dynamax"))
        };

        // Only add torment if not dynamaxed
        if !has_dynamax {
            Pokemon::add_volatile(battle, foe_pos, ID::from("torment"), Some(source_pos));
        }
    }

    EventResult::Continue
}
