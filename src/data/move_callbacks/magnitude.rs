//! Magnitude Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move, pokemon) {
///     const i = this.random(100);
///     if (i < 5) {
///         move.magnitude = 4;
///         move.basePower = 10;
///     } else if (i < 15) {
///         move.magnitude = 5;
///         move.basePower = 30;
///     } else if (i < 35) {
///         move.magnitude = 6;
///         move.basePower = 50;
///     } else if (i < 65) {
///         move.magnitude = 7;
///         move.basePower = 70;
///     } else if (i < 85) {
///         move.magnitude = 8;
///         move.basePower = 90;
///     } else if (i < 95) {
///         move.magnitude = 9;
///         move.basePower = 110;
///     } else {
///         move.magnitude = 10;
///         move.basePower = 150;
///     }
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // const i = this.random(100);
    let i = battle.random(100);

    // Determine magnitude and base power based on random value
    let (magnitude, base_power) = if i < 5 {
        (4, 10)
    } else if i < 15 {
        (5, 30)
    } else if i < 35 {
        (6, 50)
    } else if i < 65 {
        (7, 70)
    } else if i < 85 {
        (8, 90)
    } else if i < 95 {
        (9, 110)
    } else {
        (10, 150)
    };

    // Set move.basePower and move.magnitude
    battle.modify_active_move_base_power(base_power);
    battle.set_active_move_property(
        "magnitude",
        serde_json::to_value(magnitude).unwrap_or(serde_json::Value::Null),
    );

    EventResult::Continue
}

/// onUseMoveMessage(pokemon, target, move) {
///     this.add('-activate', pokemon, 'move: Magnitude', move.magnitude);
/// }
pub fn on_use_move_message(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // Get magnitude from active move property
    let magnitude = battle
        .get_active_move_property("magnitude")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;

    // this.add('-activate', pokemon, 'move: Magnitude', move.magnitude);
    let pokemon_arg = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,

            None => return EventResult::Continue,
        };

        pokemon.get_slot()
    };
    battle.add(
        "-activate",
        &[
            pokemon_arg.into(),
            "move: Magnitude".into(),
            magnitude.to_string().into(),
        ],
    );

    EventResult::Continue
}
