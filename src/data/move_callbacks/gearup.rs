//! Gear Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHitSide(side, source, move) {
///     const targets = side.allies().filter(target => (
///         target.hasAbility(['plus', 'minus']) &&
///         (!target.volatiles['maxguard'] || this.runEvent('TryHit', target, source, move))
///     ));
///     if (!targets.length) return false;
///     let didSomething = false;
///     for (const target of targets) {
///         didSomething = this.boost({ atk: 1, spa: 1 }, target, source, move, false, true) || didSomething;
///     }
///     return didSomething;
/// }
pub fn on_hit_side(battle: &mut Battle, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    use crate::dex_data::ID;
    

    let source = source_pos;

    // const targets = side.allies().filter(target => (
    //     target.hasAbility(['plus', 'minus']) &&
    //     (!target.volatiles['maxguard'] || this.runEvent('TryHit', target, source, move))
    // ));
    let mut targets = Vec::new();

    // Get the side index from source
    let side_index = match source {
        Some(pos) => pos.0,
        None => return EventResult::Continue,
    };

    // Get all active Pokemon on the same side
    let allies = battle.get_all_active(false)
        .into_iter()
        .filter(|pos| pos.0 == side_index)
        .collect::<Vec<_>>();

    for pokemon_pos in allies {
        // target.hasAbility(['plus', 'minus'])
        let has_plus_or_minus = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => continue,
            };
            pokemon.has_ability(&["plus", "minus"])
        };

        if !has_plus_or_minus {
            continue;
        }

        // (!target.volatiles['maxguard'] || this.runEvent('TryHit', target, source, move))
        let has_maxguard = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => continue,
            };
            pokemon.volatiles.contains_key(&ID::from("maxguard"))
        };

        let should_add = if has_maxguard {
            battle.run_event_bool("TryHit", Some(pokemon_pos), source, Some(&ID::from(move_id)))
        } else {
            true
        };

        if should_add {
            targets.push(pokemon_pos);
        }
    }

    // if (!targets.length) return false;
    if targets.is_empty() {
        return EventResult::Boolean(false);
    }

    // let didSomething = false;
    // for (const target of targets) {
    //     didSomething = this.boost({ atk: 1, spa: 1 }, target, source, move, false, true) || didSomething;
    // }
    let mut did_something = false;
    for target in targets {
        let boost_result = battle.boost(&[("atk", 1), ("spa", 1)], target, source, Some(move_id));
        did_something = boost_result || did_something;
    }

    // return didSomething;
    EventResult::Boolean(did_something)
}

