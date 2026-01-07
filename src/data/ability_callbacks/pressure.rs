//! Pressure Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     this.add('-ability', pokemon, 'Pressure');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add(
        "-ability",
        &[
            pokemon_ident.as_str().into(),
            "Pressure".into(),
        ],
    );
    EventResult::Continue
}

/// onDeductPP(target, source) {
///     if (target.isAlly(source)) return;
///     return 1;
/// }
pub fn on_deduct_p_p(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.isAlly(source)) return;
    if battle.is_ally(target_pos, source_pos) {
        return EventResult::Continue;
    }

    // return 1;
    EventResult::Number(1)
}

