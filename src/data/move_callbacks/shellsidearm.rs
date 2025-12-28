//! Shell Side Arm Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onPrepareHit(target, source, move) {
///     if (!source.isAlly(target)) {
///         this.attrLastMove('[anim] Shell Side Arm ' + move.category);
///     }
/// }
pub fn on_prepare_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // if (!source.isAlly(target)) {
    //     this.attrLastMove('[anim] Shell Side Arm ' + move.category);
    // }
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let is_ally = battle.is_ally(source_pos, target_pos);

    if !is_ally {
        let category = battle.active_move.as_ref().map(|m| m.category.as_str()).unwrap_or("Special");
        let anim_str = format!("[anim] Shell Side Arm {}", category);
        battle.attr_last_move(&[&anim_str]);
    }

    EventResult::Continue
}

/// onModifyMove(move, pokemon, target) {
///     if (!target) return;
///     const atk = pokemon.getStat('atk', false, true);
///     const spa = pokemon.getStat('spa', false, true);
///     const def = target.getStat('def', false, true);
///     const spd = target.getStat('spd', false, true);
///     const physical = Math.floor(Math.floor(Math.floor(Math.floor(2 * pokemon.level / 5 + 2) * 90 * atk) / def) / 50);
///     const special = Math.floor(Math.floor(Math.floor(Math.floor(2 * pokemon.level / 5 + 2) * 90 * spa) / spd) / 50);
///     if (physical > special || (physical === special && this.randomChance(1, 2))) {
///         move.category = 'Physical';
///         move.flags.contact = 1;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::StatID;

    // if (!target) return;
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const atk = pokemon.getStat('atk', false, true);
    // const spa = pokemon.getStat('spa', false, true);
    // const def = target.getStat('def', false, true);
    // const spd = target.getStat('spd', false, true);
    let (atk, spa, level) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (
            pokemon.get_stat(StatID::Atk, false),
            pokemon.get_stat(StatID::SpA, false),
            pokemon.level,
        )
    };

    let (def, spd) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (
            target.get_stat(StatID::Def, false),
            target.get_stat(StatID::SpD, false),
        )
    };

    // const physical = Math.floor(Math.floor(Math.floor(Math.floor(2 * pokemon.level / 5 + 2) * 90 * atk) / def) / 50);
    // const special = Math.floor(Math.floor(Math.floor(Math.floor(2 * pokemon.level / 5 + 2) * 90 * spa) / spd) / 50);
    let physical = (((2 * level as i32 / 5 + 2) * 90 * atk) / def) / 50;
    let special = (((2 * level as i32 / 5 + 2) * 90 * spa) / spd) / 50;

    // if (physical > special || (physical === special && this.randomChance(1, 2))) {
    //     move.category = 'Physical';
    //     move.flags.contact = 1;
    // }
    if physical > special || (physical == special && battle.random_chance(1, 2)) {
        if let Some(active_move) = &mut battle.active_move {
            active_move.category = String::from("Physical");
            active_move.flags.contact = true;
        }
    }

    EventResult::Continue
}

/// onHit(target, source, move) {
///     // Shell Side Arm normally reveals its category via animation on cart, but doesn't play either custom animation against allies
///     if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
/// }
pub fn on_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let is_ally = battle.is_ally(source_pos, target_pos);

    if !is_ally {
        let category = battle.active_move.as_ref().map(|m| m.category.as_str()).unwrap_or("Special");
        let hint_str = format!("{} Shell Side Arm", category);
        battle.hint(&hint_str, false, None);
    }

    EventResult::Continue
}

/// onAfterSubDamage(damage, target, source, move) {
///     if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
/// }
pub fn on_after_sub_damage(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), source_pos: (usize, usize), _move_id: &str) -> EventResult {
    // if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
    let is_ally = battle.is_ally(source_pos, target_pos);

    if !is_ally {
        let category = battle.active_move.as_ref().map(|m| m.category.as_str()).unwrap_or("Special");
        let hint_str = format!("{} Shell Side Arm", category);
        battle.hint(&hint_str, false, None);
    }

    EventResult::Continue
}

