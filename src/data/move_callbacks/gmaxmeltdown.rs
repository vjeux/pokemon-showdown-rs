//! G-Max Meltdown Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
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
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    effect: Option<&Effect>,
) -> EventResult {
    // for (const pokemon of source.foes()) {
    //     if (!pokemon.volatiles['dynamax']) pokemon.addVolatile('torment', source, effect);
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
        let has_dynamax = {
            let foe = match battle.pokemon_at(foe_pos.0, foe_pos.1) {
                Some(p) => p,
                None => continue,
            };
            foe.has_volatile(&ID::from("dynamax"))
        };

        // Only add torment if not dynamaxed
        // JavaScript: pokemon.addVolatile('torment', source, effect);
        if !has_dynamax {
            Pokemon::add_volatile(battle, foe_pos, ID::from("torment"), Some(source), effect, None, None);
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
    ///         onHit(source, target, effect) {
    ///                 for (const pokemon of source.foes()) {
    ///                   if (!pokemon.volatiles["dynamax"]) pokemon.addVolatile("torment", source, effect);
    ///                 }
    ///               }
    ///     },
    /// }
    /// ```
    pub fn on_hit(
        battle: &mut Battle,
        _target_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
        effect: Option<&Effect>,
    ) -> EventResult {
        // for (const pokemon of source.foes()) {
        //     if (!pokemon.volatiles["dynamax"]) pokemon.addVolatile("torment", source, effect);
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
            let has_dynamax = {
                let foe = match battle.pokemon_at(foe_pos.0, foe_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                foe.has_volatile(&ID::from("dynamax"))
            };

            // Only add torment if not dynamaxed
            // JavaScript: pokemon.addVolatile('torment', source, effect);
            if !has_dynamax {
                Pokemon::add_volatile(battle, foe_pos, ID::from("torment"), Some(source), effect, None, None);
            }
        }

        EventResult::Continue
    }
}
