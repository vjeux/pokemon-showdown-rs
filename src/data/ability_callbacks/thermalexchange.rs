//! Thermal Exchange Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (move.type === 'Fire') {
///         this.boost({ atk: 1 });
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (move.type === 'Fire')
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.move_type == "Fire" {
            // this.boost({ atk: 1 });
            battle.boost(
                &[("atk", 1)],
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

/// onUpdate(pokemon) {
///     if (pokemon.status === 'brn') {
///         this.add('-activate', pokemon, 'ability: Thermal Exchange');
///         pokemon.cureStatus();
///     }
/// }
pub fn on_update(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onSetStatus(status, target, source, effect) {
///     if (status.id !== 'brn') return;
///     if ((effect as Move)?.status) {
///         this.add('-immune', target, '[from] ability: Thermal Exchange');
///     }
///     return false;
/// }
pub fn on_set_status(_battle: &mut Battle, _status_id: &str, _target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

