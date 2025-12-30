//! Clangorous Soul Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     if (source.hp <= (source.maxhp * 33 / 100) || source.maxhp === 1) return false;
/// }
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (source.hp <= (source.maxhp * 33 / 100) || source.maxhp === 1) return false;
    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if source.hp <= (source.maxhp * 33 / 100) || source.maxhp == 1 {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onTryHit(pokemon, target, move) {
///     if (!this.boost(move.boosts!)) return null;
///     delete move.boosts;
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    // if (!this.boost(move.boosts!)) return null;
    // Get the move boosts from the active move
    let move_id = match &battle.active_move {
        Some(active_move) => active_move.id.clone(),
        None => return EventResult::Continue,
    };

    // Extract boosts before mutable borrow
    let boost_vec: Vec<(String, i8)> = {
        let move_data = match battle.dex.moves().get_by_id(&move_id) {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        if let Some(boosts) = &move_data.boosts {
            // Convert HashMap<String, i32> to Vec<(String, i8)>
            boosts.iter().map(|(k, v)| (k.clone(), *v as i8)).collect()
        } else {
            return EventResult::Boolean(true);
        }
    };

    // Apply boosts to the pokemon
    let boost_vec_refs: Vec<(&str, i8)> = boost_vec.iter().map(|(k, v)| (k.as_str(), *v)).collect();
    let boost_success = battle.boost(&boost_vec_refs, source_pos, None, None, false, false);

    if !boost_success {
        // return null;
        return EventResult::Stop;
    }

    // delete move.boosts;
    // TODO: We can't delete move.boosts from immutable move_data
    // This is handled differently in the Rust implementation

    EventResult::Continue
}

/// onHit(pokemon) {
///     this.directDamage(pokemon.maxhp * 33 / 100);
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // this.directDamage(pokemon.maxhp * 33 / 100);
    let maxhp = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.maxhp
    };

    let damage = maxhp * 33 / 100;
    battle.direct_damage(damage, Some(pokemon_pos), None, None);

    EventResult::Continue
}
