//! Rattled Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (['Dark', 'Bug', 'Ghost'].includes(move.type)) {
///         this.boost({ spe: 1 });
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (['Dark', 'Bug', 'Ghost'].includes(move.type))
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.move_type == "Dark" || move_data.move_type == "Bug" || move_data.move_type == "Ghost" {
            // this.boost({ spe: 1 });
            battle.boost(
                &[("spe", 1)],
                target_pos,
                Some(target_pos),
                None,
                false,
                false,
            );
        }
    }

    EventResult::Continue
}

/// onAfterBoost(boost, target, source, effect) {
///     if (effect?.name === 'Intimidate' && boost.atk) {
///         this.boost({ spe: 1 });
///     }
/// }
pub fn on_after_boost(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

