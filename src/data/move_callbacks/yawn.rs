//! Yawn Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onTryHit(target) {
///     if (target.status || !target.runStatusImmunity('slp')) {
///         return false;
///     }
/// }
/// JavaScript signature: onTryHit(target, source, move) - TARGET FIRST
pub fn on_try_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    _source_pos: (usize, usize),
) -> EventResult {
    let target = target_pos;

    // if (target.status || !target.runStatusImmunity('slp')) {
    //     return false;
    // }
    let (has_status, has_sleep_immunity) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let has_status = !target_pokemon.status.is_empty();
        let has_immunity = Pokemon::run_status_immunity(battle, target, "slp", false);

        (has_status, has_immunity)
    };

    if has_status || !has_sleep_immunity {
        return EventResult::NotFail;
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target, source) {
    ///     this.add('-start', target, 'move: Yawn', `[of] ${source}`);
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        let target = pokemon_pos;

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-start', target, 'move: Yawn', `[of] ${source}`);
        let (target_slot, source_slot) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (target_pokemon.get_slot(), source_pokemon.get_slot())
        };

        battle.add(
            "-start",
            &[
                crate::battle::Arg::from(target_slot),
                crate::battle::Arg::from("move: Yawn"),
                crate::battle::Arg::from(format!("[of] {}", source_slot)),
            ],
        );

        EventResult::Continue
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'move: Yawn', '[silent]');
    ///     target.trySetStatus('slp', this.effectState.source);
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        eprintln!("[YAWN_END] Called for target {:?}", target_pos);

        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // Debug: check if target already has slp
        {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            eprintln!("[YAWN_END] Target {} current status={:?}", target_pokemon.get_slot(), target_pokemon.status.as_str());
        }

        // this.add('-end', target, 'move: Yawn', '[silent]');
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
                crate::battle::Arg::from("move: Yawn"),
                crate::battle::Arg::from("[silent]"),
            ],
        );

        // target.trySetStatus('slp', this.effectState.source);
        Pokemon::try_set_status(battle, target, ID::from("slp"), None, None);

        EventResult::Continue
    }
}
