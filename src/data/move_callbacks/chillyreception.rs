//! Chilly Reception Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// priorityChargeCallback(source) {
///     source.addVolatile('chillyreception');
/// }
pub fn priority_charge_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // source.addVolatile('chillyreception');
    let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    pokemon.add_volatile(ID::from("chillyreception"));

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onBeforeMove(source, target, move) {
    ///     if (move.id !== 'chillyreception') return;
    ///     this.add('-prepare', source, 'Chilly Reception', '[premajor]');
    /// }
    pub fn on_before_move(
        battle: &mut Battle,
        source_pos: Option<(usize, usize)>,
        _target_pos: Option<(usize, usize)>,
        move_id: &str,
    ) -> EventResult {
        // if (move.id !== 'chillyreception') return;
        if move_id != "chillyreception" {
            // return;
            return EventResult::Continue;
        }

        // this.add('-prepare', source, 'Chilly Reception', '[premajor]');
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source_ident = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.get_slot()
        };

        battle.add(
            "-prepare",
            &[
                source_ident.as_str().into(),
                "Chilly Reception".into(),
                "[premajor]".into(),
            ],
        );

        EventResult::Continue
    }
}
