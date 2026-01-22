//! Adrenaline Orb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

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
    //
    // The JS comment explains:
    // - Adrenaline Orb activates if Intimidate is blocked by an ability like Hyper Cutter,
    //   which deletes boost.atk (making it undefined),
    // - but not if the holder's attack is already at -6 (or +6 if it has Contrary),
    //   which sets boost.atk to 0
    //
    // In JS, there's a difference between undefined (deleted by blocking ability) and 0 (capped).
    // In Rust, we use i8 where both cases result in 0.
    //
    // To distinguish: if boost.atk == 0 but the pokemon's attack is NOT at -6 (or +6 for Contrary),
    // then the boost was blocked (not capped), so we should still activate.
    let (target_spe_boost, target_atk_boost) = {
        let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.boosts.spe, pokemon.boosts.atk)
    };

    // Speed at +6 means the orb can't boost further
    if target_spe_boost == 6 {
        return EventResult::Continue;
    }

    // boost.atk === 0 check: This means the boost was CAPPED (at -6 or +6 limit)
    // NOT that it was blocked. In JS, blocked boosts have the key deleted (undefined).
    // We check: if boost.atk == 0 AND the pokemon IS at the limit (-6 or +6),
    // then it was capped. Otherwise, it was blocked and we should still activate.
    if boost.atk == 0 {
        // Check if the pokemon's attack is at the limit where it would be capped
        if target_atk_boost == -6 || target_atk_boost == 6 {
            // The boost was capped because at limit - don't activate
            return EventResult::Continue;
        }
        // Otherwise, boost was blocked by an ability (Own Tempo, Hyper Cutter, etc.)
        // The orb should still activate - continue to check if effect is Intimidate
    }

    // if (effect.name === 'Intimidate') {
    //     target.useItem();
    // }
    let effect_name = battle.event.as_ref().and_then(|e| {
        e.effect.as_ref().map(|eff| eff.id.as_str().to_string())
    });

    if let Some(name) = effect_name {
        if name == "intimidate" {
            // target.useItem();
            let _pokemon_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            Pokemon::use_item(battle, target_pos, None, None);
        }
    }

    EventResult::Continue
}
