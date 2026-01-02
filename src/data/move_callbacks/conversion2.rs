//! Conversion 2 Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// ```ignore
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
/// ```
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
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
                let move_data = match battle.dex.moves().get_by_id(move_id) {
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
    let mut possible_types: Vec<String> = Vec::new();

    // const attackType = target.lastMoveUsed.type;
    // for (const typeName of this.dex.types.names()) {
    let type_names: Vec<String> = battle
        .dex
        .types()
        .names()
        .iter()
        .map(|s| s.to_string())
        .collect();
    for type_name in type_names {
        // if (source.hasType(typeName)) continue;
        let has_type = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.has_type(battle, &type_name)
        };

        if has_type {
            continue;
        }

        // const typeCheck = this.dex.types.get(typeName).damageTaken[attackType];
        // if (typeCheck === 2 || typeCheck === 3) {
        //     possibleTypes.push(typeName);
        // }
        let type_check = battle
            .dex
            .types()
            .get(&type_name)
            .and_then(|type_data| type_data.damage_taken.get(&attack_type))
            .copied()
            .unwrap_or(0);
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
    let possible_types_refs: Vec<&str> = possible_types.iter().map(|s| s.as_str()).collect();
    let random_type = battle.sample(&possible_types_refs[..]);

    // if (!source.setType(randomType)) return false;
    let random_type_str = match random_type {
        Some(t) => *t,
        None => return EventResult::Boolean(false),
    };

    // Try to set the type and check if it succeeded
    let set_type_succeeded = {
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.set_type(vec![random_type_str.to_string()], false)
    };

    // Check if setType failed
    if !set_type_succeeded {
        return EventResult::Boolean(false);
    }

    // this.add('-start', source, 'typechange', randomType);
    let source_ident = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.get_slot()
    };

    battle.add(
        "-start",
        &[
            source_ident.as_str().into(),
            "typechange".into(),
            random_type_str.into(),
        ],
    );

    EventResult::Continue
}
