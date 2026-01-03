//! Wind Rider Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (pokemon.side.sideConditions['tailwind']) {
///         this.boost({ atk: 1 }, pokemon, pokemon);
///     }
/// }
pub fn on_start(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onTryHit(target, source, move) {
///     if (target !== source && move.flags['wind']) {
///         if (!this.boost({ atk: 1 }, target, target)) {
///             this.add('-immune', target, '[from] ability: Wind Rider');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(_battle: &mut Battle, _target_pos: (usize, usize), _source_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onSideConditionStart(side, source, sideCondition) {
///     const pokemon = this.effectState.target;
///     if (sideCondition.id === 'tailwind') {
///         this.boost({ atk: 1 }, pokemon, pokemon);
///     }
/// }
pub fn on_side_condition_start(battle: &mut Battle, pokemon_pos: (usize, usize), side_condition_id: &str, _source_pos: Option<(usize, usize)>) -> EventResult {
    // if (sideCondition.id === 'tailwind')
    if side_condition_id == "tailwind" {
        // this.boost({ atk: 1 }, pokemon, pokemon);
        battle.boost(&[("atk", 1)], pokemon_pos, Some(pokemon_pos), None, false, false);
    }

    EventResult::Continue
}

