//! Grass Knot Move
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
        target_pokemon.get_weight()
    };

    // let bp;
    // if (targetWeight >= 2000) { bp = 120; }
    // else if (targetWeight >= 1000) { bp = 100; }
    // else if (targetWeight >= 500) { bp = 80; }
    // else if (targetWeight >= 250) { bp = 60; }
    // else if (targetWeight >= 100) { bp = 40; }
    // else { bp = 20; }
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

/// onTryHit(target, source, move) {
///     if (target.volatiles['dynamax']) {
///         this.add('-fail', source, 'move: Grass Knot', '[from] Dynamax');
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

    // JavaScript: onTryHit(target, source, move) - target comes first, source second
    let target = target_pos;
    let source = source_pos;

    // if (target.volatiles['dynamax']) {
    let has_dynamax = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.volatiles.contains_key(&ID::from("dynamax"))
    };

    if has_dynamax {
        // this.add('-fail', source, 'move: Grass Knot', '[from] Dynamax');
        let source_ident = {
            let pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };
        battle.add(
            "-fail",
            &[
                source_ident.as_str().into(),
                "move: Grass Knot".into(),
                "[from] Dynamax".into(),
            ],
        );

        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // return null; - prevents the move from executing
        return EventResult::Null;
    }

    EventResult::Continue
}
