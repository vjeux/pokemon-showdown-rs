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
pub fn base_power_callback(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::event::EventTarget;

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const targetWeight = target.getWeight();
    // getWeight() { const weighthg = this.battle.runEvent('ModifyWeight', this, null, null, this.weighthg); return Math.max(1, weighthg); }
    let target_base_weight = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.weight_hg
    };
    let target_weight_result = battle.run_event(
        "ModifyWeight",
        Some(EventTarget::Pokemon(target)),
        None,
        None,
        EventResult::Number(target_base_weight),
        false,
        false,
    );
    let target_weight = match target_weight_result {
        EventResult::Number(w) => w.max(1),
        _ => target_base_weight.max(1),
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
pub fn on_try_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    source_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    // JavaScript: onTryHit(target, pokemon, move) - target comes first, pokemon (source) second
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
        let pokemon_arg = {
            let pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,

                None => return EventResult::Continue,
            };

            pokemon.get_slot()
        };
        battle.add("-fail", &[pokemon_arg.into(), "Dynamax".into()]);

        //     this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        //     return null; - prevents the move from executing
        return EventResult::Null;
    }

    EventResult::Continue
}
