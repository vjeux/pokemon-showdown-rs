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
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // onHit(target) {
    //     if (target.getTypes().join() === 'Water' || !target.setType('Water')) {
    //         // Soak should animate even when it fails.
    //         // Returning false would suppress the animation.
    //         this.add('-fail', target);
    //         return null;
    //     }
    //     this.add('-start', target, 'typechange', 'Water');
    // }
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.getTypes().join() === 'Water' || !target.setType('Water')) {
    let types = battle.get_types(target);
    let is_already_water = types.len() == 1 && types[0] == ID::from("Water");
    let set_type_success = battle.set_type(target, &[ID::from("Water")]);

    if is_already_water || !set_type_success {
        // Soak should animate even when it fails.
        // Returning false would suppress the animation.
        // this.add('-fail', target);
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };

        battle.add("-fail", &[target_arg]);

        // return null;
        return EventResult::Null;
    }

    // this.add('-start', target, 'typechange', 'Water');
    let target_arg = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        crate::battle::Arg::from(target_pokemon)
    };

    battle.add("-start", &[
        target_arg,
        "typechange".into(),
        "Water".into(),
    ]);

    EventResult::Continue
}

