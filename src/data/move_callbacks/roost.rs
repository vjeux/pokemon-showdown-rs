//! Roost Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

/// condition: {
///     duration: 1,
///     onResidualOrder: 25,
///     onStart(target) {
///         if (target.terastallized) {
///             if (target.hasType('Flying')) {
///                 this.add('-hint', "If a Terastallized Pokemon uses Roost, it remains Flying-type.");
///             }
///             return false;
///         }
///         this.add('-singleturn', target, 'move: Roost');
///     },
///     onTypePriority: -1,
///     onType(types, pokemon) {
///         this.effectState.typeWas = types;
///         return types.filter(type => type !== 'Flying');
///     },
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>) -> EventResult {
    

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.terastallized) {
    //     if (target.hasType('Flying')) {
    //         this.add('-hint', "If a Terastallized Pokemon uses Roost, it remains Flying-type.");
    //     }
    //     return false;
    // }
    let is_terastallized = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.terastallized.is_some()
    };

    if is_terastallized {
        let has_flying = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.has_type("Flying")
        };

        if has_flying {
            battle.add("-hint", &[crate::battle::Arg::from("If a Terastallized Pokemon uses Roost, it remains Flying-type.")]);
        }

        return EventResult::Boolean(false);
    }

    // this.add('-singleturn', target, 'move: Roost');
    let target_arg = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.get_slot()
    };

    battle.add("-singleturn", &[
        target_arg.into(),
        crate::battle::Arg::from("move: Roost"),
    ]);

    EventResult::Continue
}

/// onType(types, pokemon) {
///     this.effectState.typeWas = types;
///     return types.filter(type => type !== 'Flying');
/// }
pub fn on_type(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // this.effectState.typeWas = types;
    // return types.filter(type => type !== 'Flying');
    let types = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.get_types(false)
    };

    let filtered_types: Vec<_> = types.into_iter().filter(|t| t.as_str() != "Flying").collect();

    EventResult::Types(filtered_types)
}

}

