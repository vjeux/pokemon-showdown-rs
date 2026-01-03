//! Gale Wings Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyPriority(priority, pokemon, target, move) {
///     if (move?.type === 'Flying' && pokemon.hp === pokemon.maxhp) return priority + 1;
/// }
pub fn on_modify_priority(battle: &mut Battle, priority: i32, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // if (move?.type === 'Flying' && pokemon.hp === pokemon.maxhp) return priority + 1;

    // Check if move is Flying type
    let is_flying = battle.active_move.as_ref()
        .map(|m| m.move_type == "Flying")
        .unwrap_or(false);

    if !is_flying {
        return EventResult::Continue;
    }

    // Check if pokemon is at full HP
    let at_full_hp = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.hp == pokemon.maxhp
    };

    if at_full_hp {
        return EventResult::Number(priority + 1);
    }

    EventResult::Continue
}

