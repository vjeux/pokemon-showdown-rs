//! Magnetic Flux Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHitSide(side, source, move) {
///     const targets = side.allies().filter(ally => (
///         ally.hasAbility(['plus', 'minus']) &&
///         (!ally.volatiles['maxguard'] || this.runEvent('TryHit', ally, source, move))
///     ));
///     if (!targets.length) return false;
///
///     let didSomething = false;
///     for (const target of targets) {
///         didSomething = this.boost({ def: 1, spd: 1 }, target, source, move, false, true) || didSomething;
///     }
///     return didSomething;
/// }
pub fn on_hit_side(battle: &mut Battle, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    use crate::dex_data::ID;

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const targets = side.allies().filter(ally => (
    //     ally.hasAbility(['plus', 'minus']) &&
    //     (!ally.volatiles['maxguard'] || this.runEvent('TryHit', ally, source, move))
    // ));
    let side_index = source.0;
    // Get all allies excluding self (source)
    let allies: Vec<(usize, usize)> = battle.allies_and_self(side_index, false)
        .into_iter()
        .filter(|&pos| pos != source)
        .collect();

    let mut targets = Vec::new();
    for ally_pos in allies {
        let (has_ability, has_maxguard) = {
            let ally_pokemon = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };
            let has_ability = ally_pokemon.has_ability(&["plus"]) ||
                             ally_pokemon.has_ability(&["minus"]);
            let has_maxguard = ally_pokemon.volatiles.contains_key(&ID::from("maxguard"));
            (has_ability, has_maxguard)
        };

        if !has_ability {
            continue;
        }

        if has_maxguard {
            let try_hit_result = battle.run_event("TryHit", Some(ally_pos), Some(source), None, None);
            if try_hit_result.unwrap_or(0) == 0 {
                continue;
            }
        }

        targets.push(ally_pos);
    }

    // if (!targets.length) return false;
    if targets.is_empty() {
        return EventResult::Boolean(false);
    }

    // let didSomething = false;
    // for (const target of targets) {
    //     didSomething = this.boost({ def: 1, spd: 1 }, target, source, move, false, true) || didSomething;
    // }
    let mut did_something = false;
    for target in targets {
        let boosts = [("def", 1), ("spd", 1)];
        let boost_result = battle.boost(&boosts, target, Some(source), Some(move_id));
        did_something = boost_result || did_something;
    }

    // return didSomething;
    EventResult::Boolean(did_something)
}

