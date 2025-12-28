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
pub fn on_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // if (source.species && (source.species.num === 493 || source.species.num === 773)) return false;
    let species_check = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.species.num == 493 || source.species.num == 773
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

    // const oldApparentType = source.apparentType;
    let old_apparent_type = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.apparent_type.clone()
    };

    // let newBaseTypes = target.getTypes(true).filter(type => type !== '???');
    let mut new_base_types = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.get_types(true).into_iter().filter(|t| t.as_str() != "???").collect::<Vec<_>>()
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
            target.added_type.is_some()
        };

        if has_added_type {
            new_base_types = vec![ID::from("Normal")];
        } else {
            return EventResult::Boolean(false);
        }
    }

    // this.add('-start', source, 'typechange', '[from] move: Reflect Type', `[of] ${target}`);
    let source_arg = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        crate::battle::Arg::from(source)
    };
    let target_arg = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        crate::battle::Arg::from(target)
    };

    battle.add("-start", &[
        source_arg.clone(),
        crate::battle::Arg::from("typechange"),
        crate::battle::Arg::from("[from] move: Reflect Type"),
        crate::battle::Arg::from(format!("[of] {}", target_arg.as_string())),
    ]);

    // source.setType(newBaseTypes);
    // source.addedType = target.addedType;
    // source.knownType = target.isAlly(source) && target.knownType;
    // if (!source.knownType) source.apparentType = oldApparentType;
    let target_added_type = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.added_type.clone()
    };

    let target_known_type = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.known_type
    };

    let is_ally = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.is_ally(source)
    };

    let source = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    source.set_type(&new_base_types);
    source.added_type = target_added_type;
    source.known_type = is_ally && target_known_type;
    if !source.known_type {
        source.apparent_type = old_apparent_type;
    }

    EventResult::Continue
}

