//! G-Max Snooze Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onHit(target) {
///     if (target.status || !target.runStatusImmunity('slp')) return;
///     if (this.randomChance(1, 2)) return;
///     target.addVolatile('yawn');
/// }
/// JavaScript signature: onHit(target) - TARGET IS FIRST PARAMETER
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    let target = target_pos;

    // if (target.status || !target.runStatusImmunity('slp')) return;
    let has_status = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        !target_pokemon.status.is_empty()
    };

    if has_status {
        return EventResult::Continue;
    }

    let has_immunity = Pokemon::run_status_immunity(battle, target, "slp", false);

    if !has_immunity {
        return EventResult::Continue;
    }

    // if (this.randomChance(1, 2)) return;
    if battle.random_chance(1.0, 2) {
        return EventResult::Continue;
    }

    // target.addVolatile('yawn');
    // JavaScript: pokemon.addVolatile('yawn') uses default parameters from battle context
    // In Rust, we need to pass source and sourceEffect explicitly
    let move_effect = battle.make_move_effect(&ID::from("gmaxsnooze"));
    Pokemon::add_volatile(battle, target, ID::from("yawn"), source_pos, Some(&move_effect), None, None);

    EventResult::Continue
}

/// onAfterSubDamage(damage, target) {
///     if (target.status || !target.runStatusImmunity('slp')) return;
///     if (this.randomChance(1, 2)) return;
///     target.addVolatile('yawn');
/// }
pub fn on_after_sub_damage(
    battle: &mut Battle,
    _damage: i32,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.status || !target.runStatusImmunity('slp')) return;
    let has_status = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        !target_pokemon.status.is_empty()
    };

    if has_status {
        return EventResult::Continue;
    }

    let has_immunity = Pokemon::run_status_immunity(battle, target, "slp", false);

    if !has_immunity {
        return EventResult::Continue;
    }

    // if (this.randomChance(1, 2)) return;
    if battle.random_chance(1.0, 2) {
        return EventResult::Continue;
    }

    // target.addVolatile('yawn');
    // JavaScript: pokemon.addVolatile('yawn') uses default parameters from battle context
    // In Rust, we need to pass source and sourceEffect explicitly
    let move_effect = battle.make_move_effect(&ID::from("gmaxsnooze"));
    Pokemon::add_volatile(battle, target, ID::from("yawn"), source_pos, Some(&move_effect), None, None);

    EventResult::Continue
}
