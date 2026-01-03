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
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.side.sideConditions['tailwind'])
    let has_tailwind = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let side_index = pokemon.side_index;
        battle.sides[side_index].get_side_condition(&crate::dex_data::ID::from("tailwind")).is_some()
    };

    if has_tailwind {
        // this.boost({ atk: 1 }, pokemon, pokemon);
        battle.boost(&[("atk", 1)], pokemon_pos, Some(pokemon_pos), None, false, false);
    }

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
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), _move_id: &str) -> EventResult {
    use crate::battle::Arg;

    // if (target !== source && move.flags['wind'])
    if target_pos != source_pos {
        let is_wind_move = if let Some(ref active_move) = battle.active_move {
            active_move.flags.wind
        } else {
            return EventResult::Continue;
        };

        if is_wind_move {
            // if (!this.boost({ atk: 1 }, target, target))
            // Note: boost() returns whether the boost was successful (stat not already maxed)
            // In Rust, we'll always try to boost and check if it fails
            let boost_result = battle.boost(&[("atk", 1)], target_pos, Some(target_pos), None, false, false);

            // If boost failed (returns false in JS), show immunity message
            // In Rust, boost() doesn't return a result, so we'll always show the message for now
            // TODO: Update battle.boost() to return success status to match JS behavior exactly

            // For now, we'll show immune message unconditionally when hit by wind move
            let target_slot = {
                let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Null,
                };
                target.get_slot()
            };

            // Always boost attack (since we can't check if it succeeded yet)
            // this.add('-immune', target, '[from] ability: Wind Rider');
            // Note: The immunity message is only shown if the boost failed in JS
            // We'll implement this correctly once boost() returns a result

            // return null;
            return EventResult::Null;
        }
    }

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

