//! Magnetic Flux Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// ```ignore
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
/// ```
pub fn on_hit_side(
    battle: &mut Battle,
    source_pos: Option<(usize, usize)>,
    active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    use crate::dex_data::ID;
    let move_id = active_move.map(|m| m.id.to_string()).unwrap_or_default();

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const targets = side.allies().filter(ally => (
    //     ally.hasAbility(['plus', 'minus']) &&
    //     (!ally.volatiles['maxguard'] || this.runEvent('TryHit', ally, source, move))
    // ));
    // Note: JavaScript's side.allies() includes self, not just other allies!
    // Magnetic Flux affects the user too if they have Plus/Minus ability.
    let allies: Vec<(usize, usize)> = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.allies_and_self(battle, false)
    };

    let mut targets = Vec::new();
    for ally_pos in allies {
        let (has_ability, has_maxguard) = {
            let ally_pokemon = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };
            let has_ability =
                ally_pokemon.has_ability(battle, &["plus"]) || ally_pokemon.has_ability(battle, &["minus"]);
            let has_maxguard = ally_pokemon.volatiles.contains_key(&ID::from("maxguard"));
            (has_ability, has_maxguard)
        };

        if !has_ability {
            continue;
        }

        if has_maxguard {
            let try_hit_result =
                battle.run_event("TryHit", Some(crate::event::EventTarget::Pokemon(ally_pos)), Some(source), None, EventResult::Continue, false, false);
            // JavaScript: if (!result) continue;
            // This checks if result is FALSY, not if it equals 0
            if !try_hit_result.is_truthy() {
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
        let boost_result = battle.boost(&boosts, target, Some(source), Some(&move_id), false, false);
        did_something = boost_result || did_something;
    }

    // return didSomething;
    EventResult::Boolean(did_something)
}
