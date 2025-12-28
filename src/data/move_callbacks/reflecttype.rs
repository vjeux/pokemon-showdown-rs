//! Reflect Type Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     if (source.species && (source.species.num === 493 || source.species.num === 773)) return false;
///     if (source.terastallized) return false;
///     const oldApparentType = source.apparentType;
///     let newBaseTypes = target.getTypes(true).filter(type => type !== '???');
///     if (!newBaseTypes.length) {
///         if (target.addedType) {
///             newBaseTypes = ['Normal'];
///         } else {
///             return false;
///         }
///     }
///     this.add('-start', source, 'typechange', '[from] move: Reflect Type', `[of] ${target}`);
///     source.setType(newBaseTypes);
///     source.addedType = target.addedType;
///     source.knownType = target.isAlly(source) && target.knownType;
///     if (!source.knownType) source.apparentType = oldApparentType;
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (source.species && (source.species.num === 493 || source.species.num === 773)) return false;
    let species_num = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        // Look up species data to get the num field
        battle.dex.get_species_by_id(&source_pokemon.species_id)
            .map(|s| s.num)
            .unwrap_or(0)
    };

    if species_num == 493 || species_num == 773 {
        return EventResult::Boolean(false);
    }

    // if (source.terastallized) return false;
    let terastallized = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.terastallized
    };

    if terastallized.is_some() {
        return EventResult::Boolean(false);
    }

    // const oldApparentType = source.apparentType;
    // apparentType is types.join('/') in JavaScript
    let old_apparent_type = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.types.join("/")
    };

    // let newBaseTypes = target.getTypes(true).filter(type => type !== '???');
    let new_base_types = {
        let target_types = battle.get_types(target, true);
        target_types.into_iter()
            .filter(|t| t != "???")
            .collect::<Vec<_>>()
    };

    // if (!newBaseTypes.length) {
    //     if (target.addedType) {
    //         newBaseTypes = ['Normal'];
    //     } else {
    //         return false;
    //     }
    // }
    let new_base_types = if new_base_types.is_empty() {
        let target_added_type = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.added_type.clone()
        };

        if target_added_type.is_some() {
            vec!["Normal".to_string()]
        } else {
            return EventResult::Boolean(false);
        }
    } else {
        new_base_types
    };

    // this.add('-start', source, 'typechange', '[from] move: Reflect Type', `[of] ${target}`);
    let (source_arg, target_arg) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (crate::battle::Arg::from(source_pokemon), crate::battle::Arg::from(target_pokemon))
    };

    battle.add("-start", &[
        source_arg,
        "typechange".into(),
        "[from] move: Reflect Type".into(),
        format!("[of] {}", target_arg).into(),
    ]);

    // source.setType(newBaseTypes);
    battle.set_type(source, &new_base_types);

    // source.addedType = target.addedType;
    let target_added_type = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.added_type.clone()
    };

    let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    source_pokemon.added_type = target_added_type;

    // source.knownType = target.isAlly(source) && target.knownType;
    let is_ally = battle.is_ally(target, source);
    let target_known_type = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.known_type
    };

    let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    source_pokemon.known_type = if is_ally { target_known_type } else { None };

    // if (!source.knownType) source.apparentType = oldApparentType;
    // apparentType is computed from types, no need to store it separately
    // The old types have already been set above, so no action needed here

    EventResult::Continue
}

