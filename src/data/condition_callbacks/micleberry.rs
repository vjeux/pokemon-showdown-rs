//! Micle Berry Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! The Micle Berry volatile is added when the berry is eaten.
//! It grants a 1.2x accuracy boost to the next move used.

use crate::battle::Battle;
use crate::battle::Effect;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onSourceAccuracy(accuracy, target, source, move) {
///     if (!move.ohko) {
///         this.add('-enditem', source, 'Micle Berry');
///         source.removeVolatile('micleberry');
///         if (typeof accuracy === 'number') {
///             return this.chainModify([4915, 4096]);
///         }
///     }
/// }
pub fn on_source_accuracy(
    battle: &mut Battle,
    accuracy: i32,
    _target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    debug_elog!("[MICLEBERRY_ON_SOURCE_ACCURACY] Called with accuracy={:?}, source_pos={:?}", accuracy, source_pos);

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!move.ohko) { ... }
    let is_ohko = active_move.map(|m| m.ohko.is_some()).unwrap_or(false);
    if is_ohko {
        return EventResult::Continue;
    }

    // this.add('-enditem', source, 'Micle Berry');
    let source_slot = {
        let pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };
    battle.add("-enditem", &[source_slot.into(), "Micle Berry".into()]);

    // source.removeVolatile('micleberry');
    Pokemon::remove_volatile(battle, source, &ID::from("micleberry"));

    // if (typeof accuracy === 'number') { return this.chainModify([4915, 4096]); }
    // The accuracy parameter is already a number if we got here
    // chainModify([4915, 4096]) is approximately 1.2x accuracy
    if accuracy > 0 {
        battle.chain_modify_fraction(4915, 4096);
    }

    EventResult::Continue
}
