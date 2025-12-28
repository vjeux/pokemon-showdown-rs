//! G-Max Snooze Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target) {
///     if (target.status || !target.runStatusImmunity('slp')) return;
///     if (this.randomChance(1, 2)) return;
///     target.addVolatile('yawn');
/// }
pub fn on_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

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

    let has_immunity = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.run_status_immunity("slp")
    };

    if !has_immunity {
        return EventResult::Continue;
    }

    // if (this.randomChance(1, 2)) return;
    if battle.random_chance(1, 2) {
        return EventResult::Continue;
    }

    // target.addVolatile('yawn');
    let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    target_pokemon.add_volatile(ID::from("yawn"));

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
) -> EventResult {
    use crate::dex_data::ID;

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

    let has_immunity = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.run_status_immunity("slp")
    };

    if !has_immunity {
        return EventResult::Continue;
    }

    // if (this.randomChance(1, 2)) return;
    if battle.random_chance(1, 2) {
        return EventResult::Continue;
    }

    // target.addVolatile('yawn');
    let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    target_pokemon.add_volatile(ID::from("yawn"));

    EventResult::Continue
}
