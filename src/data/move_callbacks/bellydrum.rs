//! Belly Drum Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target) {
///     if (target.hp <= target.maxhp / 2 || target.boosts.atk >= 6 || target.maxhp === 1) { // Shedinja clause
///         return false;
///     }
///     this.directDamage(target.maxhp / 2);
///     this.boost({ atk: 12 }, target);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let (hp, maxhp, atk_boost) = if let Some(target) = battle.pokemon_at(target_pos.0, target_pos.1) {
        (target.hp, target.maxhp, target.boosts.atk)
    } else {
        return EventResult::Continue;
    };

    // Fail if HP is too low, attack is already maxed, or maxhp is 1 (Shedinja clause)
    if hp <= maxhp / 2 || atk_boost >= 6 || maxhp == 1 {
        return EventResult::Bool(false);
    }

    // Deal damage equal to half of max HP
    battle.direct_damage(maxhp / 2, target_pos, None, None);

    // Boost attack by 12 stages (max is 6, so this maxes it out)
    battle.boost(&[("atk", 12)], target_pos, Some(pokemon_pos), None);

    EventResult::Continue
}

