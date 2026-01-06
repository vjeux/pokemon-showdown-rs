//! Reflect Type Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

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
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),           // FIXED: First positional param is target (the Pokemon being hit)
    source_pos: Option<(usize, usize)>,   // FIXED: Second positional param is source (the user of the move)
) -> EventResult {
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (source.species && (source.species.num === 493 || source.species.num === 773)) return false;
    // Species 493 is Arceus, 773 is Silvally
    let species_check = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let species = source.species_id.as_str();
        species.starts_with("arceus") || species.starts_with("silvally")
    };

    if species_check {
        return EventResult::Boolean(false);
    }

    // if (source.terastallized) return false;
    let is_terastallized = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.terastallized.is_some()
    };

    if is_terastallized {
        return EventResult::Boolean(false);
    }

    // let newBaseTypes = target.getTypes(true).filter(type => type !== '???');
    let mut new_base_types = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target
            .get_types(battle, true)
            .into_iter()
            .filter(|t| t.as_str() != "???")
            .collect::<Vec<_>>()
    };

    // if (!newBaseTypes.length) {
    //     if (target.addedType) {
    //         newBaseTypes = ['Normal'];
    //     } else {
    //         return false;
    //     }
    // }
    if new_base_types.is_empty() {
        let has_added_type = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            // JavaScript: addedType is string, check if not empty
            !target.added_type.is_empty()
        };

        if has_added_type {
            new_base_types = vec![String::from("Normal")];
        } else {
            return EventResult::Boolean(false);
        }
    }

    // this.add('-start', source, 'typechange', '[from] move: Reflect Type', `[of] ${target}`);
    let (source_ident, target_ident) = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source.get_slot(), target.get_slot())
    };

    battle.add(
        "-start",
        &[
            source_ident.clone().into(),
            "typechange".into(),
            "[from] move: Reflect Type".into(),
            format!("[of] {}", target_ident).into(),
        ],
    );

    // source.setType(newBaseTypes);
    // source.addedType = target.addedType;
    let target_added_type = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.added_type.clone()
    };

    Pokemon::set_type(battle, source_pos, new_base_types, false);

    let source = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    source.added_type = target_added_type;

    EventResult::Continue
}
