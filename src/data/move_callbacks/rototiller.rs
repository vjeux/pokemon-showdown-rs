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
    let mut any_airborne = false;

    // for (const pokemon of this.getAllActive()) {
    let all_active = battle.get_all_active(false);

    for pokemon_pos in all_active {
        // if (!pokemon.runImmunity('Ground')) {
        //     this.add('-immune', pokemon);
        //     anyAirborne = true;
        //     continue;
        // }
        let is_immune_to_ground = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => continue,
            };
            !pokemon.run_immunity("Ground")
        };

        if is_immune_to_ground {
            let pokemon_slot = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                pokemon.get_slot()
            };
            battle.add("-immune", &[crate::battle::Arg::from(pokemon_slot)]);
            any_airborne = true;
            continue;
        }

        // if (pokemon.hasType('Grass')) {
        //     // This move affects every grounded Grass-type Pokemon in play.
        //     targets.push(pokemon);
        // }
        let has_grass = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => continue,
            };
            pokemon.has_type(battle, "Grass")
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
        battle.boost(&[("atk", 1), ("spa", 1)], pokemon_pos, source, None, false, false);
    }

    EventResult::Continue
}
