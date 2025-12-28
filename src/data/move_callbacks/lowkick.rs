//! Low Kick Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target) {
///     const targetWeight = target.getWeight();
///     let bp;
///     if (targetWeight >= 2000) {
///         bp = 120;
///     } else if (targetWeight >= 1000) {
///         bp = 100;
///     } else if (targetWeight >= 500) {
///         bp = 80;
///     } else if (targetWeight >= 250) {
///         bp = 60;
///     } else if (targetWeight >= 100) {
///         bp = 40;
///     } else {
///         bp = 20;
///     }
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const targetWeight = target.getWeight();
    let target_weight = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.get_weight(battle)
    };

    // let bp;
    // if (targetWeight >= 2000) {
    //     bp = 120;
    // } else if (targetWeight >= 1000) {
    //     bp = 100;
    // } else if (targetWeight >= 500) {
    //     bp = 80;
    // } else if (targetWeight >= 250) {
    //     bp = 60;
    // } else if (targetWeight >= 100) {
    //     bp = 40;
    // } else {
    //     bp = 20;
    // }
    let bp = if target_weight >= 2000 {
        120
    } else if target_weight >= 1000 {
        100
    } else if target_weight >= 500 {
        80
    } else if target_weight >= 250 {
        60
    } else if target_weight >= 100 {
        40
    } else {
        20
    };

    // this.debug(`BP: ${bp}`);
    battle.debug(&format!("BP: {}", bp));

    // return bp;
    EventResult::Number(bp)
}

/// onTryHit(target, pokemon, move) {
///     if (target.volatiles['dynamax']) {
///         this.add('-fail', pokemon, 'Dynamax');
///         this.attrLastMove('[still]');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = source_pos;
    let target = target_pos;

    // if (target.volatiles['dynamax']) {
    let has_dynamax = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.volatiles.contains_key(&ID::from("dynamax"))
    };

    if has_dynamax {
        //     this.add('-fail', pokemon, 'Dynamax');
        let pokemon_arg = crate::battle::Arg::Pos(pokemon.0, pokemon.1);
        battle.add("-fail", &[pokemon_arg, "Dynamax".into()]);

        //     this.attrLastMove('[still]');
        battle.attr_last_move("[still]");

        //     return null;
        return EventResult::Stop;
    }

    EventResult::Continue
}

