//! Core Enforcer Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onHit(target) {
///     if (target.getAbility().flags['cantsuppress']) return;
///     if (target.newlySwitched || this.queue.willMove(target)) return;
///     target.addVolatile('gastroacid');
/// }
pub fn on_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.getAbility().flags['cantsuppress']) return;
    let cant_suppress = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let ability_id = &target_pokemon.ability;
        match battle.dex.abilities().get(ability_id.as_str()) {
            Some(ability_data) => ability_data.flags.contains_key("cantsuppress"),
            None => false,
        }
    };

    if cant_suppress {
        // return;
        return EventResult::Continue;
    }

    // if (target.newlySwitched || this.queue.willMove(target)) return;
    let newly_switched = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.newly_switched
    };

    let will_move = battle.queue.will_move(target.0, target.1);

    if newly_switched || will_move.is_some() {
        // return;
        return EventResult::Continue;
    }

    // target.addVolatile('gastroacid');
    let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    target_pokemon.add_volatile(ID::from("gastroacid"));

    EventResult::Continue
}

/// onAfterSubDamage(damage, target) {
///     if (target.getAbility().flags['cantsuppress']) return;
///     if (target.newlySwitched || this.queue.willMove(target)) return;
///     target.addVolatile('gastroacid');
/// }
pub fn on_after_sub_damage(
    battle: &mut Battle,
    _damage: i32,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.getAbility().flags['cantsuppress']) return;
    let cant_suppress = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let ability_id = &target_pokemon.ability;
        match battle.dex.abilities().get(ability_id.as_str()) {
            Some(ability_data) => ability_data.flags.contains_key("cantsuppress"),
            None => false,
        }
    };

    if cant_suppress {
        // return;
        return EventResult::Continue;
    }

    // if (target.newlySwitched || this.queue.willMove(target)) return;
    let newly_switched = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.newly_switched
    };

    let will_move = battle.queue.will_move(target.0, target.1);

    if newly_switched || will_move.is_some() {
        // return;
        return EventResult::Continue;
    }

    // target.addVolatile('gastroacid');
    let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    target_pokemon.add_volatile(ID::from("gastroacid"));

    EventResult::Continue
}
