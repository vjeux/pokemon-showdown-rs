//! Sparkly Swirl Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(pokemon, source, move)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(pokemon, source, move) {
/// 			this.add('-activate', source, 'move: Aromatherapy');
/// 			for (const ally of source.side.pokemon) {
/// 				if (ally !== source && (ally.volatiles['substitute'] && !move.infiltrates)) {
/// 					continue;
/// 				}
/// 				ally.cureStatus();
/// 			}
/// 		},
///
/// 	}
/// ```
pub fn on_hit(
    battle: &mut Battle,
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // this.add('-activate', source, 'move: Aromatherapy');
    let source_ident = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.get_slot()
    };
    battle.add(
        "-activate",
        &[source_ident.as_str().into(), "move: Aromatherapy".into()],
    );

    // for (const ally of source.side.pokemon) {
    let source_side_idx = source.0;
    let num_pokemon = battle.sides[source_side_idx].pokemon.len();

    for ally_idx in 0..num_pokemon {
        let ally_pos = (source_side_idx, ally_idx);

        // if (ally !== source && (ally.volatiles['substitute'] && !move.infiltrates)) {
        //     continue;
        // }
        if ally_pos != source {
            let has_substitute = {
                let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                ally.volatiles.contains_key(&ID::from("substitute"))
            };

            // if (ally.volatiles['substitute'] && !move.infiltrates) continue;
            if has_substitute {
                let infiltrates = battle
                    .active_move
                    .as_ref()
                    .map(|m| m.infiltrates)
                    .unwrap_or(false);
                if !infiltrates {
                    continue;
                }
            }
        }

        // ally.cureStatus();
        Pokemon::cure_status(battle, ally_pos, false);
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
    ///         onHit(pokemon, source, move) {
    ///                 this.add("-activate", source, "move: Aromatherapy");
    ///                 for (const ally of source.side.pokemon) {
    ///                   if (ally !== source && (ally.volatiles["substitute"] && !move.infiltrates)) {
    ///                     continue;
    ///                   }
    ///                   ally.cureStatus();
    ///                 }
    ///               }
    ///     },
    /// }
    /// ```
    pub fn on_hit(
        battle: &mut Battle,
        _target_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
