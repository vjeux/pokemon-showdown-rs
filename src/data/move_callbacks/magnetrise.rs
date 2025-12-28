//! Magnet Rise Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source, target, move) {
///     if (target.volatiles['smackdown'] || target.volatiles['ingrain']) return false;
///
///     // Additional Gravity check for Z-move variant
///     if (this.field.getPseudoWeather('Gravity')) {
///         this.add('cant', source, 'move: Gravity', move);
///         return null;
///     }
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.volatiles['smackdown'] || target.volatiles['ingrain']) return false;
    let has_smackdown_or_ingrain = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.volatiles.contains_key(&ID::from("smackdown")) ||
        target_pokemon.volatiles.contains_key(&ID::from("ingrain"))
    };

    if has_smackdown_or_ingrain {
        return EventResult::Boolean(false);
    }

    // // Additional Gravity check for Z-move variant
    // if (this.field.getPseudoWeather('Gravity')) {
    //     this.add('cant', source, 'move: Gravity', move);
    //     return null;
    // }
    let has_gravity = battle.field.pseudo_weather.contains_key(&ID::from("gravity"));
    if has_gravity {
        let source_arg = {

            let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {

                Some(p) => p,

                None => return EventResult::Continue,

            };

            pokemon.get_slot()

        };
        let move_id = match &battle.active_move {
            Some(active_move) => active_move.id.clone(),
            None => ID::from(""),
        };
        battle.add("cant", &[source_arg, "move: Gravity".into(), move_id.to_string().into()]);
        return EventResult::Stop;
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-start', target, 'Magnet Rise');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-start', target, 'Magnet Rise');
        let target_arg = {

            let pokemon = match battle.pokemon_at(target.0, target.1) {

                Some(p) => p,

                None => return EventResult::Continue,

            };

            pokemon.get_slot()

        };
        battle.add("-start", &[target_arg, "Magnet Rise".into()]);

        EventResult::Continue
    }

    /// onImmunity(type) {
    ///     if (type === 'Ground') return false;
    /// }
    pub fn on_immunity(battle: &mut Battle) -> EventResult {
        // TODO: This callback needs the type parameter to work correctly
        // The TypeScript version checks: if (type === 'Ground') return false;
        // Once the event system supports passing the type parameter, implement:
        // if type_id == "ground" { return EventResult::Boolean(false); }
        EventResult::Continue
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'Magnet Rise');
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-end', target, 'Magnet Rise');
        let target_arg = {

            let pokemon = match battle.pokemon_at(target.0, target.1) {

                Some(p) => p,

                None => return EventResult::Continue,

            };

            pokemon.get_slot()

        };
        battle.add("-end", &[target_arg, "Magnet Rise".into()]);

        EventResult::Continue
    }
}
