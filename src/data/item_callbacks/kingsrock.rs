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

    use crate::dex::MoveSecondary;

    if let Some(ref active_move) = battle.active_move {
        // if (move.category !== "Status")
        if active_move.borrow().category != "Status" {
            // Check if flinch already exists in secondaries
            // for (const secondary of move.secondaries) {
            //     if (secondary.volatileStatus === 'flinch') return;
            // }
            let has_flinch = active_move.borrow().secondaries.iter()
                .any(|s| s.volatile_status.as_deref() == Some("flinch"));

            if has_flinch {
                return EventResult::Continue;
            }

            // move.secondaries.push({
            //     chance: 10,
            //     volatileStatus: 'flinch',
            // });
            active_move.borrow_mut().secondaries.push(MoveSecondary {
                chance: Some(10),
                volatile_status: Some("flinch".to_string()),
                ..Default::default()
            });
        }
    }

    EventResult::Continue
}
