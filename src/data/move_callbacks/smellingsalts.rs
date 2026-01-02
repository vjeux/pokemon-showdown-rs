//! Smelling Salts Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// basePowerCallback(pokemon, target, move) {
///     if (target.status === 'par') {
///         this.debug('BP doubled on paralyzed target');
///         return move.basePower * 2;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // basePowerCallback(pokemon, target, move) {
    //     if (target.status === 'par') {
    //         this.debug('BP doubled on paralyzed target');
    //         return move.basePower * 2;
    //     }
    //     return move.basePower;
    // }
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.status === 'par') {
    let has_paralysis = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.status == ID::from("par")
    };

    if has_paralysis {
        // this.debug('BP doubled on paralyzed target');
        battle.debug("BP doubled on paralyzed target");

        // return move.basePower * 2;
        let base_power = {
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            active_move.base_power
        };

        return EventResult::Number(base_power * 2);
    }

    // return move.basePower;
    let base_power = {
        let active_move = match &battle.active_move {
            Some(active_move) => active_move,
            None => return EventResult::Continue,
        };
        active_move.base_power
    };

    EventResult::Number(base_power)
}

/// onHit(target) {
///     if (target.status === 'par') target.cureStatus();
/// }
pub fn on_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // onHit(target) {
    //     if (target.status === 'par') target.cureStatus();
    // }
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.status === 'par') target.cureStatus();
    let (has_paralysis, target_ident, target_name) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (target_pokemon.status == ID::from("par"), target_pokemon.get_slot(), target_pokemon.name.clone())
    };

    if has_paralysis {
        let target_mut = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some((status, removed_nightmare, _silent)) = Pokemon::cure_status(battle, target, false) {
            let full_name = format!("{}: {}", target_ident, target_name);
            battle.add("-curestatus", &[full_name.as_str().into(), status.as_str().into(), "[msg]".into()]);
            if removed_nightmare {
                battle.add("-end", &[full_name.as_str().into(), "Nightmare".into(), "[silent]".into()]);
            }
        }
    }

    EventResult::Continue
}
