//! Burning Jealousy Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(target, source, move)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(target, source, move) {
/// 				if (target?.statsRaisedThisTurn) {
/// 					target.trySetStatus('brn', source, move);
/// 				}
/// 			},
///
/// 		}
/// ```
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (target?.statsRaisedThisTurn) {
    //     target.trySetStatus('brn', source, move);
    // }

    let stats_raised = {
        let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.stats_raised_this_turn
    };

    if stats_raised {
        let _target_pokemon = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let move_effect = crate::battle::Effect::move_("burningjealousy");
        Pokemon::try_set_status(battle, target_pos, ID::from("brn"), Some(&move_effect));
    }

    EventResult::Continue
}
