//! Gooey Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (this.checkMoveMakesContact(move, source, target, true)) {
///         this.add('-ability', target, 'Gooey');
///         this.boost({ spe: -1 }, source, target, null, true);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (this.checkMoveMakesContact(move, source, target, true))
    if battle.check_move_makes_contact(&crate::ID::from(move_id), target_pos, source_pos, true) {
        // this.add('-ability', target, 'Gooey');
        let target_slot = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.get_slot()
        };

        battle.add(
            "-ability",
            &[
                crate::battle::Arg::from(target_slot),
                crate::battle::Arg::from("Gooey"),
            ],
        );

        // this.boost({ spe: -1 }, source, target, null, true);
        battle.boost(
            &[("spe", -1)],
            source_pos,
            Some(target_pos),
            None,
            true,
            false,
        );
    }

    EventResult::Continue
}

