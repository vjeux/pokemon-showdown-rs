//! Stench Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     if (move.category !== "Status") {
///         this.debug('Adding Stench flinch');
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
pub fn on_modify_move(battle: &mut Battle, _move_id: &str, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::battle_actions::SecondaryEffect;

    // if (move.category !== "Status")
    let is_status = if let Some(ref active_move) = battle.active_move {
        active_move.category == "Status"
    } else {
        return EventResult::Continue;
    };

    if is_status {
        return EventResult::Continue;
    }

    // this.debug('Adding Stench flinch');
    // if (!move.secondaries) move.secondaries = [];
    // for (const secondary of move.secondaries) {
    //     if (secondary.volatileStatus === 'flinch') return;
    // }
    if let Some(ref mut active_move) = battle.active_move {
        // Check if flinch secondary already exists
        for secondary in &active_move.secondaries {
            if secondary.volatile_status.as_ref().map(|s| s.as_str()) == Some("flinch") {
                return EventResult::Continue;
            }
        }

        // move.secondaries.push({ chance: 10, volatileStatus: 'flinch' });
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

    EventResult::Continue
}

