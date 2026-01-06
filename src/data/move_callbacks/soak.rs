//! Soak Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(target) {
///     if (target.getTypes().join() === 'Water' || !target.setType('Water')) {
///         // Soak should animate even when it fails.
///         // Returning false would suppress the animation.
///         this.add('-fail', target);
///         return null;
///     }
///     this.add('-start', target, 'typechange', 'Water');
/// }
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
) -> EventResult {
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
        let types = target.get_types(battle, false);
        types.len() == 1 && types[0].as_str() == "Water"
    };

    // Try to set the type and check if it succeeded
    let set_type_succeeded = Pokemon::set_type(battle, target_pos, vec![String::from("Water")], false);

    // Check if it failed (either already Water or setType returned false)
    if types_are_water || !set_type_succeeded {
        let target_ident = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.get_slot()
        };
        battle.add("-fail", &[target_ident.into()]);
        return EventResult::Stop;
    }

    // this.add('-start', target, 'typechange', 'Water');
    let target_ident = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.get_slot()
    };

    battle.add(
        "-start",
        &[target_ident.into(), "typechange".into(), "Water".into()],
    );

    EventResult::Continue
}
