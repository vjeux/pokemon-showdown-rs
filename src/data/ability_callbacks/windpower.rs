//! Wind Power Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (move.flags['wind']) {
///         target.addVolatile('charge');
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    use crate::Pokemon;

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (move.flags['wind'])
    let is_wind_move = if let Some(ref active_move) = battle.active_move {
        active_move.flags.wind
    } else {
        return EventResult::Continue;
    };

    if is_wind_move {
        // target.addVolatile('charge');
        Pokemon::add_volatile(battle, target_pos, crate::dex_data::ID::from("charge"), None, None, None,
            None);
    }

    EventResult::Continue
}

/// onSideConditionStart(side, source, sideCondition) {
///     const pokemon = this.effectState.target;
///     if (sideCondition.id === 'tailwind') {
///         pokemon.addVolatile('charge');
///     }
/// }
pub fn on_side_condition_start(battle: &mut Battle, pokemon_pos: (usize, usize), _side_idx: usize, side_condition_id: &str, _source_pos: Option<(usize, usize)>) -> EventResult {
    use crate::Pokemon;

    // if (sideCondition.id === 'tailwind')
    if side_condition_id == "tailwind" {
        // pokemon.addVolatile('charge');
        Pokemon::add_volatile(battle, pokemon_pos, crate::dex_data::ID::from("charge"), None, None, None,
            None);
    }

    EventResult::Continue
}

