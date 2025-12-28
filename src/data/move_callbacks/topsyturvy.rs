//! Topsy-Turvy Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target) {
///     let success = false;
///     let i: BoostID;
///     for (i in target.boosts) {
///         if (target.boosts[i] === 0) continue;
///         target.boosts[i] = -target.boosts[i];
///         success = true;
///     }
///     if (!success) return false;
///     this.add('-invertboost', target, '[from] move: Topsy-Turvy');
/// }
pub fn on_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // let success = false;
    // for (i in target.boosts) {
    //     if (target.boosts[i] === 0) continue;
    //     target.boosts[i] = -target.boosts[i];
    //     success = true;
    // }
    let success = {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let mut any_inverted = false;

        // Invert all non-zero boosts
        if target_pokemon.boosts.atk != 0 {
            target_pokemon.boosts.atk = -target_pokemon.boosts.atk;
            any_inverted = true;
        }
        if target_pokemon.boosts.def != 0 {
            target_pokemon.boosts.def = -target_pokemon.boosts.def;
            any_inverted = true;
        }
        if target_pokemon.boosts.spa != 0 {
            target_pokemon.boosts.spa = -target_pokemon.boosts.spa;
            any_inverted = true;
        }
        if target_pokemon.boosts.spd != 0 {
            target_pokemon.boosts.spd = -target_pokemon.boosts.spd;
            any_inverted = true;
        }
        if target_pokemon.boosts.spe != 0 {
            target_pokemon.boosts.spe = -target_pokemon.boosts.spe;
            any_inverted = true;
        }
        if target_pokemon.boosts.accuracy != 0 {
            target_pokemon.boosts.accuracy = -target_pokemon.boosts.accuracy;
            any_inverted = true;
        }
        if target_pokemon.boosts.evasion != 0 {
            target_pokemon.boosts.evasion = -target_pokemon.boosts.evasion;
            any_inverted = true;
        }

        any_inverted
    };

    // if (!success) return false;
    if !success {
        return EventResult::NotFail;
    }

    // this.add('-invertboost', target, '[from] move: Topsy-Turvy');
    let target_arg = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.get_slot()
    };

    battle.add(
        "-invertboost",
        &[target_arg.into(), "[from] move: Topsy-Turvy".into()],
    );

    EventResult::Continue
}
