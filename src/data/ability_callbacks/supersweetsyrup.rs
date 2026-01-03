//! Supersweet Syrup Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (pokemon.syrupTriggered) return;
///     pokemon.syrupTriggered = true;
///     this.add('-ability', pokemon, 'Supersweet Syrup');
///     for (const target of pokemon.adjacentFoes()) {
///         if (target.volatiles['substitute']) {
///             this.add('-immune', target);
///         } else {
///             this.boost({ evasion: -1 }, target, pokemon, null, true);
///         }
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;

    // if (pokemon.syrupTriggered) return;
    let already_triggered = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.data.get("syrupTriggered").map(|v| v.as_bool()).flatten().unwrap_or(false)
    };

    if already_triggered {
        return EventResult::Continue;
    }

    // pokemon.syrupTriggered = true;
    {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.data.insert("syrupTriggered".to_string(), serde_json::json!(true));
    }

    // this.add('-ability', pokemon, 'Supersweet Syrup');
    let pokemon_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-ability", &[Arg::String(pokemon_slot), Arg::Str("Supersweet Syrup")]);

    // for (const target of pokemon.adjacentFoes())
    let adjacent_foes = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.adjacent_foes(battle)
    };

    for target_pos in adjacent_foes {
        // if (target.volatiles['substitute'])
        let has_substitute = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };
            target.has_volatile(&ID::from("substitute"))
        };

        if has_substitute {
            // this.add('-immune', target);
            let target_slot = {
                let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                target.get_slot()
            };
            battle.add("-immune", &[Arg::String(target_slot)]);
        } else {
            // this.boost({ evasion: -1 }, target, pokemon, null, true);
            battle.boost(&[("evasion", -1)], target_pos, Some(pokemon_pos), None, true, false);
        }
    }

    EventResult::Continue
}

