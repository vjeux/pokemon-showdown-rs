//! Fell Stinger Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterMoveSecondarySelf(pokemon, target, move) {
///     if (!target || target.fainted || target.hp <= 0) this.boost({ atk: 3 }, pokemon, pokemon, move);
/// }
pub fn on_after_move_secondary_self(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    use std::collections::HashMap;

    let pokemon = pokemon_pos;

    // if (!target || target.fainted || target.hp <= 0) this.boost({ atk: 3 }, pokemon, pokemon, move);
    let should_boost = if let Some(target) = target_pos {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.fainted || target_pokemon.hp <= 0
    } else {
        true  // No target means we should boost
    };

    if should_boost {
        // this.boost({ atk: 3 }, pokemon, pokemon, move);
        let mut boosts = HashMap::new();
        boosts.insert("atk".to_string(), 3);

        battle.boost(boosts, pokemon, Some(pokemon), Some(move_id));
    }

    EventResult::Continue
}

