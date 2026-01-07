//! G-Max Finale Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onHit(target, source, move)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(target, source, move) {
/// 				for (const pokemon of source.alliesAndSelf()) {
/// 					this.heal(pokemon.maxhp / 6, pokemon, source, move);
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
    // for (const pokemon of source.alliesAndSelf()) {
    //     this.heal(pokemon.maxhp / 6, pokemon, source, move);
    // }

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_side = source.0;

    // Get all allies and self on the same side
    let ally_positions: Vec<(usize, usize)> = battle
        .get_all_active(false)
        .into_iter()
        .filter(|(side_idx, _)| *side_idx == source_side)
        .collect();

    let move_id = ID::from("gmaxfinale");

    for ally_pos in ally_positions {
        // Get max HP for this pokemon
        let max_hp = {
            let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };
            ally.maxhp
        };

        // Heal 1/6 of max HP
        let heal_amount = max_hp / 6;
        battle.heal(heal_amount, Some(ally_pos), Some(source), Some(&move_id));
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
    ///         onHit(target, source, move) {
    ///                 for (const pokemon of source.alliesAndSelf()) {
    ///                   this.heal(pokemon.maxhp / 6, pokemon, source, move);
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
        // for (const pokemon of source.alliesAndSelf()) {
        //     this.heal(pokemon.maxhp / 6, pokemon, source, move);
        // }

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let ally_positions = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.allies_and_self(battle, false)
        };

        let move_id = ID::from("gmaxfinale");

        for ally_pos in ally_positions {
            // Get max HP for this pokemon
            let max_hp = {
                let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                ally.maxhp
            };

            // Heal 1/6 of max HP
            let heal_amount = max_hp / 6;
            battle.heal(heal_amount, Some(ally_pos), Some(source), Some(&move_id));
        }

        EventResult::Continue
    }
}
