//! Strength Sap Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     if (target.boosts.atk === -6) return false;
///     const atk = target.getStat('atk', false, true);
///     const success = this.boost({ atk: -1 }, target, source, null, false, true);
///     return !!(this.heal(atk, source, target) || success);
/// }
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),  // First param is target
    source_pos: Option<(usize, usize)>,  // Second param is source
) -> EventResult {
    use crate::dex_data::StatID;

    let target = target_pos;
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.boosts.atk === -6) return false;
    let atk_boost = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.boosts.atk
    };

    // if (target.boosts.atk === -6) return false;
    // JS returns false (boolean), not NOT_FAIL
    if atk_boost == -6 {
        return EventResult::Boolean(false);
    }

    // const atk = target.getStat('atk', false, true);
    let atk = battle.get_pokemon_stat(target, StatID::Atk, false, true);

    // const success = this.boost({ atk: -1 }, target, source, null, false, true);
    let success = battle.boost(&[("atk", -1)], target, Some(source), None, false, false);

    // return !!(this.heal(atk, source, target) || success);
    // JS: this.heal(atk, source, target) where:
    //   - source = Pokemon using Strength Sap (being healed)
    //   - target = Pokemon whose attack was drained (source of HP drain)
    // Rust heal signature: heal(damage, target_to_heal, source_of_heal, effect)
    let heal_result = battle.heal(atk, Some(source), Some(target), None);

    if heal_result.is_some() || success {
        EventResult::Continue
    } else {
        EventResult::NotFail
    }
}
