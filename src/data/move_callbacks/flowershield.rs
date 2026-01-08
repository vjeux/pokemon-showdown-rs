//! Flower Shield Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onHitField(t, source, move) {
///     const targets: Pokemon[] = [];
///     for (const pokemon of this.getAllActive()) {
///         if (
///             pokemon.hasType('Grass') &&
///             (!pokemon.volatiles['maxguard'] ||
///                 this.runEvent('TryHit', pokemon, source, move))
///         ) {
///             // This move affects every Grass-type Pokemon in play.
///             targets.push(pokemon);
///         }
///     }
///     let success = false;
///     for (const target of targets) {
///         success = this.boost({ def: 1 }, target, source, move) || success;
///     }
///     return success;
/// }
pub fn on_hit_field(
    battle: &mut Battle,
    source_pos: Option<(usize, usize)>,
    active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    use crate::dex_data::ID;
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");

    let source = source_pos;

    // const targets: Pokemon[] = [];
    // for (const pokemon of this.getAllActive()) {
    let mut targets = Vec::new();
    let all_active = battle.get_all_active(false);

    for pokemon_pos in all_active {
        // if (
        //     pokemon.hasType('Grass') &&
        //     (!pokemon.volatiles['maxguard'] ||
        //         this.runEvent('TryHit', pokemon, source, move))
        // ) {
        let has_grass_type = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => continue,
            };
            pokemon.has_type(battle, "grass")
        };

        if !has_grass_type {
            continue;
        }

        let has_maxguard = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => continue,
            };
            pokemon.volatiles.contains_key(&ID::from("maxguard"))
        };

        let should_add = if has_maxguard {
            // this.runEvent('TryHit', pokemon, source, move)
            battle.run_event(
                "TryHit",
                Some(crate::event::EventTarget::Pokemon(pokemon_pos)),
                source,
                Some(&Effect::move_(ID::from(move_id))),
                crate::event::EventResult::Number(1),
                false,
                false,
            ).is_truthy()
        } else {
            true
        };

        if should_add {
            // This move affects every Grass-type Pokemon in play.
            // targets.push(pokemon);
            targets.push(pokemon_pos);
        }
    }

    // let success = false;
    // for (const target of targets) {
    //     success = this.boost({ def: 1 }, target, source, move) || success;
    // }
    let mut success = false;
    for target in targets {
        let boost_result = battle.boost(&[("def", 1)], target, source, Some(move_id), false, false);
        success = boost_result || success;
    }

    // return success;
    EventResult::Boolean(success)
}
