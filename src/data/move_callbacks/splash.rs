//! Splash Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source, target, move) {
///     // Additional Gravity check for Z-move variant
///     if (this.field.getPseudoWeather('Gravity')) {
///         this.add('cant', source, 'move: Gravity', move);
///         return null;
///     }
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // onTry(source, target, move) {
    //     // Additional Gravity check for Z-move variant
    //     if (this.field.getPseudoWeather('Gravity')) {
    //         this.add('cant', source, 'move: Gravity', move);
    //         return null;
    //     }
    // }
    let source = source_pos;

    // if (this.field.getPseudoWeather('Gravity')) {
    let has_gravity = battle.field.has_pseudo_weather(&ID::from("gravity"));

    if has_gravity {
        // this.add('cant', source, 'move: Gravity', move);
        let (source_arg, move_arg) = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let active_move = match &battle.active_move {
                Some(active_move) => active_move.id.as_str(),
                None => return EventResult::Continue,
            };

            (source_pokemon.get_slot(), crate::battle::Arg::Str(active_move))
        };

        battle.add("cant", &[
            source_arg,
            "move: Gravity".into(),
            move_arg,
        ]);

        // return null;
        return EventResult::Stop;
    }

    EventResult::Continue
}

/// onTryHit(target, source) {
///     this.add('-nothing');
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    // onTryHit(target, source) {
    //     this.add('-nothing');
    // }

    // this.add('-nothing');
    battle.add("-nothing", &[]);

    EventResult::Continue
}

