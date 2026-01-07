//! G-Max Stun Shock Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(source)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(source) {
/// 				for (const pokemon of source.foes()) {
/// 					const result = this.random(2);
/// 					if (result === 0) {
/// 						pokemon.trySetStatus('par', source);
/// 					} else {
/// 						pokemon.trySetStatus('psn', source);
/// 					}
/// 				}
/// 			},
///
/// 		}
/// ```
pub fn on_hit(
    battle: &mut Battle,
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // for (const pokemon of source.foes()) {
    //     const result = this.random(2);
    //     if (result === 0) {
    //         pokemon.trySetStatus('par', source);
    //     } else {
    //         pokemon.trySetStatus('psn', source);
    //     }
    // }

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let foe_positions = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.foes(battle, false)
    };

    for foe_pos in foe_positions {
        let result = battle.random(2);

        if result == 0 {
            Pokemon::try_set_status(battle, foe_pos, ID::from("par"), None);
        } else {
            Pokemon::try_set_status(battle, foe_pos, ID::from("psn"), None);
        }
    }

    EventResult::Continue
}


/// Self-targeting callbacks
/// These callbacks target the move user (source), not the move target
pub mod self_callbacks {
    use super::*;

    /// self.onHit(source)
    ///
    /// ```text
    /// JS Source (data/moves.ts):
    /// self: {
    ///     onHit(source) {
    ///         onHit(source) {
    ///                 for (const pokemon of source.foes()) {
    ///                   const result = this.random(2);
    ///                   if (result === 0) {
    ///                     pokemon.trySetStatus("par", source);
    ///                   } else {
    ///                     pokemon.trySetStatus("psn", source);
    ///                   }
    ///                 }
    ///               }
    ///     },
    /// }
    /// ```
    pub fn on_hit(
        battle: &mut Battle,
        _target_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // for (const pokemon of source.foes()) {
        //     const result = this.random(2);
        //     if (result === 0) {
        //         pokemon.trySetStatus("par", source);
        //     } else {
        //         pokemon.trySetStatus("psn", source);
        //     }
        // }

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let foe_positions = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.foes(battle, false)
        };

        for foe_pos in foe_positions {
            let result = battle.random(2);

            if result == 0 {
                Pokemon::try_set_status(battle, foe_pos, ID::from("par"), None);
            } else {
                Pokemon::try_set_status(battle, foe_pos, ID::from("psn"), None);
            }
        }

        EventResult::Continue
    }
}
