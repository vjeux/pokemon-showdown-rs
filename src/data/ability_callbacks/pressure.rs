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
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
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
pub fn on_deduct_p_p(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

