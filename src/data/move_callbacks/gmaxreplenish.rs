//! G-Max Replenish Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(source)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(source) {
/// 				if (this.randomChance(1, 2)) return;
/// 				for (const pokemon of source.alliesAndSelf()) {
/// 					if (pokemon.item) continue;
///
/// 					if (pokemon.lastItem && this.dex.items.get(pokemon.lastItem).isBerry) {
/// 						const item = pokemon.lastItem;
/// 						pokemon.lastItem = '';
/// 						this.add('-item', pokemon, this.dex.items.get(item), '[from] move: G-Max Replenish');
/// 						pokemon.setItem(item);
/// 					}
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
    use crate::dex_data::ID;

    // if (this.randomChance(1, 2)) return;
    if battle.random_chance(1, 2) {
        return EventResult::Continue;
    }

    // for (const pokemon of source.alliesAndSelf()) {
    // Get all active pokemon and filter for allies and self
    let all_active = battle.get_all_active(false);
    let source_side = source_pos.0;

    for pokemon_pos in all_active {
        // Only process pokemon on the same side as source (allies and self)
        if pokemon_pos.0 != source_side {
            continue;
        }

        //     if (pokemon.item) continue;
        let has_item = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => continue,
            };
            !pokemon.get_item().is_empty()
        };

        if has_item {
            continue;
        }

        //     if (pokemon.lastItem && this.dex.items.get(pokemon.lastItem).isBerry) {
        let (last_item_id, is_berry) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => continue,
            };

            let last_item = pokemon.last_item.clone();
            if last_item.is_empty() {
                continue;
            }

            let is_berry = battle.dex.items().get_by_id(&last_item)
                .map(|item| item.is_berry)
                .unwrap_or(false);

            (last_item, is_berry)
        };

        if !is_berry {
            continue;
        }

        //         const item = pokemon.lastItem;
        //         pokemon.lastItem = '';
        //         this.add('-item', pokemon, this.dex.items.get(item), '[from] move: G-Max Replenish');
        //         pokemon.setItem(item);
        let item_name = battle.dex.items().get_by_id(&last_item_id)
            .map(|i| i.name.clone())
            .unwrap_or_else(|| last_item_id.to_string());

        let pokemon_slot = {
            let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => continue,
            };
            pokemon.last_item = ID::empty();
            pokemon.get_slot()
        };

        battle.add(
            "-item",
            &[
                crate::battle::Arg::from(pokemon_slot),
                crate::battle::Arg::from(item_name),
                crate::battle::Arg::from("[from] move: G-Max Replenish"),
            ],
        );

        Pokemon::set_item(battle, pokemon_pos, last_item_id, None, None);
    //     }
    // }
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
    ///                 if (this.randomChance(1, 2)) return;
    ///                 for (const pokemon of source.alliesAndSelf()) {
    ///                   if (pokemon.item) continue;
    ///                   if (pokemon.lastItem && this.dex.items.get(pokemon.lastItem).isBerry) {
    ///                     const item = pokemon.lastItem;
    ///                     pokemon.lastItem = "";
    ///                     this.add("-item", pokemon, this.dex.items.get(item), "[from] move: G-Max Replenish");
    ///                     pokemon.setItem(item);
    ///                   }
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
