//! Conversion 2 Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;
use crate::dex_data::ID;

/// onHit(target, source) {
///     if (!target.lastMoveUsed) {
///         return false;
///     }
///     const possibleTypes = [];
///     const attackType = target.lastMoveUsed.type;
///     for (const typeName of this.dex.types.names()) {
///         if (source.hasType(typeName)) continue;
///         const typeCheck = this.dex.types.get(typeName).damageTaken[attackType];
///         if (typeCheck === 2 || typeCheck === 3) {
///             possibleTypes.push(typeName);
///         }
///     }
///     if (!possibleTypes.length) {
///         return false;
///     }
///     const randomType = this.sample(possibleTypes);
///
///     if (!source.setType(randomType)) return false;
///     this.add('-start', source, 'typechange', randomType);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Get target and source
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };
    let source = pokemon_pos;

    // if (!target.lastMoveUsed) {
    //     return false;
    // }
    let attack_type = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        match &target_pokemon.last_move_used {
            Some(move_id) => {
                let move_data = match battle.dex.get_move_by_id(move_id) {
                    Some(m) => m,
                    None => return EventResult::Boolean(false),
                };
                move_data.move_type.clone()
            }
            None => {
                return EventResult::Boolean(false);
            }
        }
    };

    // const possibleTypes = [];
    let mut possible_types: Vec<&str> = Vec::new();

    // const attackType = target.lastMoveUsed.type;
    // for (const typeName of this.dex.types.names()) {
    let type_names = battle.dex.get_type_names();
    for type_name in type_names {
        // if (source.hasType(typeName)) continue;
        let has_type = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.has_type(&type_name)
        };

        if has_type {
            continue;
        }

        // const typeCheck = this.dex.types.get(typeName).damageTaken[attackType];
        // if (typeCheck === 2 || typeCheck === 3) {
        //     possibleTypes.push(typeName);
        // }
        let type_check = battle.dex.get_type_damage_taken(&type_name, &attack_type);
        if type_check == 2 || type_check == 3 {
            possible_types.push(type_name);
        }
    }

    // if (!possibleTypes.length) {
    //     return false;
    // }
    if possible_types.is_empty() {
        return EventResult::Boolean(false);
    }

    // const randomType = this.sample(possibleTypes);
    let random_type = battle.sample(&possible_types[..]);

    // if (!source.setType(randomType)) return false;
    let random_type_str = match random_type {
        Some(t) => t.clone(),
        None => return EventResult::Boolean(false),
    };

    {
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.set_type(vec![random_type_str.clone()]);
    }

    // this.add('-start', source, 'typechange', randomType);
    let source_arg = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Arg::from(source_pokemon)
    };

    battle.add("-start", &[source_arg, "typechange".into(), random_type_str.into()]);

    EventResult::Continue
}
