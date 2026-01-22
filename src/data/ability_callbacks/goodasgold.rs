//! Good as Gold Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (move.category === 'Status' && target !== source) {
///         this.add('-immune', target, '[from] ability: Good as Gold');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.category (the active move's category, not the dex category)
    let is_status = active_move.map(|m| m.category == "Status").unwrap_or(false);

    if target_pos != source_pos && is_status {
        let target_ident = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Null,
            };
            target.get_slot()
        };

        battle.add(
            "-immune",
            &[
                target_ident.as_str().into(),
                "[from] ability: Good as Gold".into(),
            ],
        );
        return EventResult::Null;
    }
    EventResult::Continue
}

