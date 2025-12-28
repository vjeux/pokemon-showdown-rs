//! Flower Shield Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
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
pub fn on_hit_field(battle: &mut Battle, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    use crate::dex_data::ID;
    

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
            pokemon.has_type("grass")
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
            battle.run_event_bool("TryHit", Some(pokemon_pos), source, Some(&ID::from(move_id)))
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
        let boost_result = battle.boost(&[("def", 1)], target, source, Some(move_id));
        success = boost_result || success;
    }

    // return success;
    EventResult::Boolean(success)
}

