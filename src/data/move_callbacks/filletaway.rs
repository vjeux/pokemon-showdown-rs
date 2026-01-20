//! Fillet Away Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     if (source.hp <= source.maxhp / 2 || source.maxhp === 1) return false;
/// }
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let source = source_pos;

    // if (source.hp <= source.maxhp / 2 || source.maxhp === 1) return false;
    let (hp, max_hp) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source_pokemon.hp, source_pokemon.maxhp)
    };

    if hp <= max_hp / 2 || max_hp == 1 {
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
    let pokemon = source_pos;

    // if (!this.boost(move.boosts!)) return null;
    // boosts: { atk: 2, spa: 2, spe: 2 }
    let boost_result = battle.boost(
        &[("atk", 2), ("spa", 2), ("spe", 2)],
        pokemon,
        Some(pokemon),
        None,
        false,
        false,
    );

    if !boost_result {
        // JS: return null; - prevents on_hit from running
        return EventResult::Null;
    }

    // delete move.boosts;
    // We need to clear the boosts from the current move
    if let Some(ref mut active_move) = battle.active_move {
        active_move.boosts = None;
    }

    EventResult::Continue
}

/// onHit(pokemon) {
///     this.directDamage(pokemon.maxhp / 2);
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // this.directDamage(pokemon.maxhp / 2);
    let (max_hp, hp_before) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon_pokemon.maxhp, pokemon_pokemon.hp)
    };

    battle.direct_damage(max_hp / 2, Some(pokemon), None, None);

    // Check for Emergency Exit after direct damage
    // JavaScript: if (pokemon.hp <= pokemon.maxhp / 2 && hpBeforeDamage > pokemon.maxhp / 2)
    //             this.battle.runEvent('EmergencyExit', pokemon, pokemon);
    let hp_after = battle.pokemon_at(pokemon.0, pokemon.1)
        .map(|p| p.hp)
        .unwrap_or(0);
    if hp_after <= max_hp / 2 && hp_before > max_hp / 2 {
        battle.run_event("EmergencyExit", Some(crate::event::EventTarget::Pokemon(pokemon)), Some(pokemon), None, EventResult::Continue, false, false);
    }

    EventResult::Continue
}
