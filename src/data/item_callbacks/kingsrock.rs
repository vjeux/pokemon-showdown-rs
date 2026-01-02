//! King's Rock Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     if (move.category !== "Status") {
///         if (!move.secondaries) move.secondaries = [];
///         for (const secondary of move.secondaries) {
///             if (secondary.volatileStatus === 'flinch') return;
///         }
///         move.secondaries.push({
///             chance: 10,
///             volatileStatus: 'flinch',
///         });
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.category !== "Status") {
    //     if (!move.secondaries) move.secondaries = [];
    //     for (const secondary of move.secondaries) {
    //         if (secondary.volatileStatus === 'flinch') return;
    //     }
    //     move.secondaries.push({
    //         chance: 10,
    //         volatileStatus: 'flinch',
    //     });
    // }

    use crate::battle_actions::SecondaryEffect;

    if let Some(active_move) = battle.active_move.as_mut() {
        // if (move.category !== "Status")
        if active_move.category != "Status" {
            // Check if flinch already exists in secondaries
            // for (const secondary of move.secondaries) {
            //     if (secondary.volatileStatus === 'flinch') return;
            // }
            let has_flinch = active_move.secondaries.iter()
                .any(|s| s.volatile_status.as_deref() == Some("flinch"));

            if has_flinch {
                return EventResult::Continue;
            }

            // move.secondaries.push({
            //     chance: 10,
            //     volatileStatus: 'flinch',
            // });
            active_move.secondaries.push(SecondaryEffect {
                chance: Some(10),
                boosts: None,
                status: None,
                volatile_status: Some("flinch".to_string()),
                side_condition: None,
                slot_condition: None,
                pseudo_weather: None,
                terrain: None,
                weather: None,
                ability: None,
                kingsrock: None,
                self_effect: false,
            });
        }
    }

    EventResult::Continue
}
