//! Weakness Policy Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onDamagingHit(damage, target, source, move) {
///     if (!move.damage && !move.damageCallback && target.getMoveHitData(move).typeMod > 0) {
///         target.useItem();
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), _source_pos: (usize, usize)) -> EventResult {
    // if (!move.damage && !move.damageCallback && target.getMoveHitData(move).typeMod > 0) {
    //     target.useItem();
    // }

    // Check if move has fixed damage (move.damage) or custom damage calculation (move.damageCallback)
    let has_custom_damage = match &battle.active_move {
        Some(active_move) => {
            // Check if move has fixed damage value (like Dragon Rage's 40 damage)
            if active_move.borrow().damage.is_some() {
                true
            } else {
                // Check if move has damage callback (like Counter, Seismic Toss)
                // These are the moves registered in move_callbacks/mod.rs::dispatch_damage_callback
                matches!(active_move.borrow().id.as_str(),
                    "comeuppance" | "counter" | "endeavor" | "finalgambit" |
                    "guardianofalola" | "metalburst" | "mirrorcoat" | "naturesmadness" |
                    "psywave" | "ruination" | "superfang"
                )
            }
        },
        None => return EventResult::Continue,
    };

    // If move has custom damage, don't trigger Weakness Policy
    if has_custom_damage {
        return EventResult::Continue;
    }

    // Clone the active_move for run_effectiveness (need owned copy to avoid borrow issues)
    let active_move_clone = match &battle.active_move {
        Some(active_move) => active_move.borrow().clone(),
        None => return EventResult::Continue,
    };

    // Check type effectiveness against target
    // target.getMoveHitData(move).typeMod > 0 means super effective
    let type_effectiveness = Pokemon::run_effectiveness(battle, target_pos, &active_move_clone);

    // typeMod > 0 in JavaScript means super effective
    if type_effectiveness > 0 {
        // target.useItem();
        let _target_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::use_item(battle, target_pos, None, None);
    }

    EventResult::Continue
}
