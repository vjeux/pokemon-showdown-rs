//! Adrenaline Orb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterBoost(boost, target, source, effect) {
///     // Adrenaline Orb activates if Intimidate is blocked by an ability like Hyper Cutter,
///     // which deletes boost.atk,
///     // but not if the holder's attack is already at -6 (or +6 if it has Contrary),
///     // which sets boost.atk to 0
///     if (target.boosts['spe'] === 6 || boost.atk === 0) {
///         return;
///     }
///     if (effect.name === 'Intimidate') {
///         target.useItem();
///     }
/// }
pub fn on_after_boost(
    battle: &mut Battle,
    target_pos: (usize, usize),
    boost: &crate::dex_data::BoostsTable,
) -> EventResult {
    // if (target.boosts['spe'] === 6 || boost.atk === 0) {
    //     return;
    // }
    let target_spe_boost = {
        let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.boosts.spe
    };

    if target_spe_boost == 6 || boost.atk == 0 {
        return EventResult::Continue;
    }

    // if (effect.name === 'Intimidate') {
    //     target.useItem();
    // }
    let effect_name = battle.current_event.as_ref().and_then(|e| {
        e.effect.as_ref().map(|id| id.as_str().to_string())
    });

    if let Some(name) = effect_name {
        if name == "intimidate" {
            // target.useItem();
            let pokemon_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_mut.use_item();
        }
    }

    EventResult::Continue
}
