//! Rototiller Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHitField(target, source) {
///     const targets: Pokemon[] = [];
///     let anyAirborne = false;
///     for (const pokemon of this.getAllActive()) {
///         if (!pokemon.runImmunity('Ground')) {
///             this.add('-immune', pokemon);
///             anyAirborne = true;
///             continue;
///         }
///         if (pokemon.hasType('Grass')) {
///             // This move affects every grounded Grass-type Pokemon in play.
///             targets.push(pokemon);
///         }
///     }
///     if (!targets.length && !anyAirborne) return false; // Fails when there are no grounded Grass types or airborne Pokemon
///     for (const pokemon of targets) {
///         this.boost({ atk: 1, spa: 1 }, pokemon, source);
///     }
/// }
pub fn on_hit_field(
    battle: &mut Battle,
    _target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    let source = source_pos;

    // const targets: Pokemon[] = [];
    // let anyAirborne = false;
    let mut targets: Vec<(usize, usize)> = Vec::new();
    let any_airborne = false;

    // for (const pokemon of this.getAllActive()) {
    let all_active = battle.get_all_active(false);

    for pokemon_pos in all_active {
        // if (!pokemon.runImmunity('Ground')) {
        //     this.add('-immune', pokemon);
        //     anyAirborne = true;
        //     continue;
        // }
        // TODO: Implement immunity check when run_immunity is available

        // if (pokemon.hasType('Grass')) {
        //     // This move affects every grounded Grass-type Pokemon in play.
        //     targets.push(pokemon);
        // }
        let has_grass = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.has_type("Grass")
        };

        if has_grass {
            targets.push(pokemon_pos);
        }
    }

    // if (!targets.length && !anyAirborne) return false; // Fails when there are no grounded Grass types or airborne Pokemon
    if targets.is_empty() && !any_airborne {
        return EventResult::Boolean(false);
    }

    // for (const pokemon of targets) {
    //     this.boost({ atk: 1, spa: 1 }, pokemon, source);
    // }
    for pokemon_pos in targets {
        battle.boost(&[("atk", 1), ("spa", 1)], pokemon_pos, source, None);
    }

    EventResult::Continue
}
