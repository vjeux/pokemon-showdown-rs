//! Telekinesis Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source, target, move) {
///     // Additional Gravity check for Z-move variant
///     if (this.field.getPseudoWeather('Gravity')) {
///         this.attrLastMove('[still]');
///         this.add('cant', source, 'move: Gravity', move);
///         return null;
///     }
/// }
pub fn on_try(
    _battle: &mut Battle,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     if (['Diglett', 'Dugtrio', 'Palossand', 'Sandygast'].includes(target.baseSpecies.baseSpecies) ||
    ///         target.baseSpecies.name === 'Gengar-Mega') {
    ///         this.add('-immune', target);
    ///         return null;
    ///     }
    ///     if (target.volatiles['smackdown'] || target.volatiles['ingrain']) return false;
    ///     this.add('-start', target, 'Telekinesis');
    /// }
    pub fn on_start(_battle: &mut Battle, _target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onAccuracy(accuracy, target, source, move) {
    ///     if (move && !move.ohko) return true;
    /// }
    pub fn on_accuracy(
        _battle: &mut Battle,
        _accuracy: i32,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        _move_id: &str,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onImmunity(type) {
    ///     if (type === 'Ground') return false;
    /// }
    pub fn on_immunity(_battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onUpdate(pokemon) {
    ///     if (pokemon.baseSpecies.name === 'Gengar-Mega') {
    ///         delete pokemon.volatiles['telekinesis'];
    ///         this.add('-end', pokemon, 'Telekinesis', '[silent]');
    ///     }
    /// }
    pub fn on_update(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'Telekinesis');
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-end', target, 'Telekinesis');
        let target_slot = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-end",
            &[
                crate::battle::Arg::from(target_slot),
                crate::battle::Arg::from("Telekinesis"),
            ],
        );

        EventResult::Continue
    }
}
