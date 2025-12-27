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
    // Get the target (self for belly drum)
    let target = target_pos.unwrap_or(pokemon_pos);

    // Check conditions
    let (should_fail, maxhp) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // if (target.hp <= target.maxhp / 2 || target.boosts.atk >= 6 || target.maxhp === 1) {
        let should_fail = target_pokemon.hp <= target_pokemon.maxhp / 2
            || target_pokemon.boosts.atk >= 6
            || target_pokemon.maxhp == 1;

        (should_fail, target_pokemon.maxhp)
    };

    if should_fail {
        // return false;
        return EventResult::Bool(false);
    }

    // this.directDamage(target.maxhp / 2);
    battle.direct_damage(maxhp / 2, Some(target), None, None);

    // this.boost({ atk: 12 }, target);
    battle.boost(&[("atk", 12)], target, None, None);

    EventResult::Continue
}

