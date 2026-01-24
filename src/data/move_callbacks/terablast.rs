//! Tera Blast Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     if (pokemon.terastallized === 'Stellar') {
///         return 100;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the pokemon
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get the active move
    let active_move = match &battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (pokemon.terastallized === 'Stellar')
    if pokemon.terastallized.as_deref() == Some("Stellar") {
        return EventResult::Number(100);
    }

    // return move.basePower;
    EventResult::Number(active_move.borrow().base_power)
}

/// onPrepareHit(target, source, move) {
///     if (source.terastallized) {
///         this.attrLastMove('[anim] Tera Blast ' + source.teraType);
///     }
/// }
pub fn on_prepare_hit(
    battle: &mut Battle,
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // JS: onPrepareHit(target, source, move) - source is second param
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (source.terastallized)
    let tera_type = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if source_pokemon.terastallized.is_some() {
            // this.attrLastMove('[anim] Tera Blast ' + source.teraType);
            source_pokemon.tera_type.clone()
        } else {
            return EventResult::Continue;
        }
    };

    if let Some(tera_type_str) = tera_type {
        let anim_attr = format!("[anim] Tera Blast {}", tera_type_str);
        battle.attr_last_move(&[&anim_attr]);
    }

    EventResult::Continue
}

/// onModifyType(move, pokemon, target) {
///     if (pokemon.terastallized) {
///         move.type = pokemon.teraType;
///     }
/// }
pub fn on_modify_type(
    battle: &mut Battle,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // if (pokemon.terastallized)
    let tera_type = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if pokemon_ref.terastallized.is_some() {
            // move.type = pokemon.teraType;
            pokemon_ref.tera_type.clone()
        } else {
            return EventResult::Continue;
        }
    };

    // Set the move type to the tera type
    if let Some(tera_type_str) = tera_type {
        if let Some(ref mut active_move) = battle.active_move {
            active_move.borrow_mut().move_type = tera_type_str;
        }
    }

    EventResult::Continue
}

/// onModifyMove(move, pokemon) {
///     if (pokemon.terastallized && pokemon.getStat('atk', false, true) > pokemon.getStat('spa', false, true)) {
///         move.category = 'Physical';
///     }
///     if (pokemon.terastallized === 'Stellar') {
///         move.self = { boosts: { atk: -1, spa: -1 } };
///     }
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::StatID;

    let pokemon = pokemon_pos;

    // if (pokemon.terastallized && pokemon.getStat('atk', false, true) > pokemon.getStat('spa', false, true))
    let (terastallized_value, should_be_physical) = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let terastallized = pokemon_ref.terastallized.clone();

        if terastallized.is_some() {
            // pokemon.getStat('atk', false, true) > pokemon.getStat('spa', false, true)
            // Second parameter false = apply boosts, third parameter true = don't run Modify events
            let atk_stat = battle.get_pokemon_stat(pokemon, StatID::Atk, false, true);
            let spa_stat = battle.get_pokemon_stat(pokemon, StatID::SpA, false, true);
            (terastallized, atk_stat > spa_stat)
        } else {
            (None, false)
        }
    };

    if let Some(ref mut active_move) = battle.active_move {
        // if (pokemon.terastallized && ...)
        if terastallized_value.is_some() && should_be_physical {
            // move.category = 'Physical';
            active_move.borrow_mut().category = "Physical".to_string();
        }

        // if (pokemon.terastallized === 'Stellar')
        if terastallized_value.as_deref() == Some("Stellar") {
            // move.self = { boosts: { atk: -1, spa: -1 } };
            let mut boosts = crate::dex_data::BoostsTable::default();
            boosts.atk = -1;
            boosts.spa = -1;
            active_move.borrow_mut().self_boost = Some(boosts);
        }
    }

    EventResult::Continue
}
