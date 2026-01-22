//! Intimidate Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onStart(pokemon) {
///     let activated = false;
///     for (const target of pokemon.adjacentFoes()) {
///         if (!activated) {
///             this.add('-ability', pokemon, 'Intimidate', 'boost');
///             activated = true;
///         }
///         if (target.volatiles['substitute']) {
///             this.add('-immune', target);
///         } else {
///             this.boost({ atk: -1 }, target, pokemon, null, true);
///         }
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    use crate::battle::Arg;
    use crate::dex_data::ID;

    // for (const target of pokemon.adjacentFoes())
    let adjacent_foes = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.adjacent_foes(battle)
    };

    if adjacent_foes.is_empty() {
        return EventResult::Continue;
    }

    // let activated = false;
    let mut activated = false;

    for target_pos in adjacent_foes {
        // if (!activated)
        if !activated {
            let pokemon_id = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                let side_id = format!("p{}", pokemon.side_index + 1);
                if pokemon.is_active {
                    let pos_letter = (b'a' + pokemon.position as u8) as char;
                    format!("{}{}: {}", side_id, pos_letter, pokemon.name)
                } else {
                    format!("{}: {}", side_id, pokemon.name)
                }
            };

            // this.add('-ability', pokemon, 'Intimidate', 'boost');
            battle.add("-ability", &[
                Arg::String(pokemon_id),
                Arg::Str("Intimidate"),
                Arg::Str("boost"),
            ]);
            activated = true;
        }

        // if (target.volatiles['substitute'])
        let has_substitute = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };
            let substitute_id = ID::from("substitute");
            target.volatiles.contains_key(&substitute_id)
        };

        if has_substitute {
            let target_id = {
                let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                let side_id = format!("p{}", target.side_index + 1);
                if target.is_active {
                    let pos_letter = (b'a' + target.position as u8) as char;
                    format!("{}{}: {}", side_id, pos_letter, target.name)
                } else {
                    format!("{}: {}", side_id, target.name)
                }
            };

            // this.add('-immune', target);
            battle.add("-immune", &[Arg::String(target_id)]);
        } else {
            // this.boost({ atk: -1 }, target, pokemon, null, true);
            battle.boost(
                &[("atk", -1)],
                target_pos,
                Some(pokemon_pos),
                None,
                false,
                true,
            );
        }
    }

    EventResult::Continue
}

