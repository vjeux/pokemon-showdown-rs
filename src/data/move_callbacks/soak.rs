//! Soak Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target) {
///     if (target.getTypes().join() === 'Water' || !target.setType('Water')) {
///         // Soak should animate even when it fails.
///         // Returning false would suppress the animation.
///         this.add('-fail', target);
///         return null;
///     }
///     this.add('-start', target, 'typechange', 'Water');
/// }
pub fn on_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // if (target.getTypes().join() === 'Water' || !target.setType('Water')) {
    //     // Soak should animate even when it fails.
    //     // Returning false would suppress the animation.
    //     this.add('-fail', target);
    //     return null;
    // }
    let types_are_water = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let types = target.get_types(false);
        types.len() == 1 && types[0].as_str() == "Water"
    };

    let set_type_result = {
        let target = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.set_type(&[ID::from("Water")])
    };

    if types_are_water || !set_type_result {
        let target_arg = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target)
        };
        battle.add("-fail", &[target_arg]);
        return EventResult::Null;
    }

    // this.add('-start', target, 'typechange', 'Water');
    let target_arg = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        crate::battle::Arg::from(target)
    };

    battle.add("-start", &[
        target_arg,
        crate::battle::Arg::from("typechange"),
        crate::battle::Arg::from("Water"),
    ]);

    EventResult::Continue
}

