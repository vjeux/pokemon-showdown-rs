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
pub fn on_prepare_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (!source.isAlly(target)) {
    //     this.attrLastMove('[anim] Shell Side Arm ' + move.category);
    // }

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let is_ally = battle.is_ally(source, target_pos);

    if !is_ally {
        let category = battle
            .active_move
            .as_ref()
            .map(|m| m.borrow().category.to_string())
            .unwrap_or_else(|| "Special".to_string());
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
pub fn on_modify_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
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
    let level = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.level
    };

    let atk = battle.get_pokemon_stat(pokemon_pos, StatID::Atk, false, true);
    let spa = battle.get_pokemon_stat(pokemon_pos, StatID::SpA, false, true);
    let def = battle.get_pokemon_stat(target_pos, StatID::Def, false, true);
    let spd = battle.get_pokemon_stat(target_pos, StatID::SpD, false, true);

    // const physical = Math.floor(Math.floor(Math.floor(Math.floor(2 * pokemon.level / 5 + 2) * 90 * atk) / def) / 50);
    // const special = Math.floor(Math.floor(Math.floor(Math.floor(2 * pokemon.level / 5 + 2) * 90 * spa) / spd) / 50);
    let physical = (((2 * level as i32 / 5 + 2) * 90 * atk) / def) / 50;
    let special = (((2 * level as i32 / 5 + 2) * 90 * spa) / spd) / 50;

    // if (physical > special || (physical === special && this.randomChance(1, 2))) {
    //     move.category = 'Physical';
    //     move.flags.contact = 1;
    // }
    if physical > special || (physical == special && battle.random_chance(1.0, 2)) {
        if let Some(ref active_move) = battle.active_move {
            active_move.borrow_mut().category = String::from("Physical");
            active_move.borrow_mut().flags.contact = true;
        }
    }

    EventResult::Continue
}

/// onHit(target, source, move) {
///     // Shell Side Arm normally reveals its category via animation on cart, but doesn't play either custom animation against allies
///     if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
/// }
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let is_ally = battle.is_ally(source, target_pos);

    if !is_ally {
        let category = battle
            .active_move
            .as_ref()
            .map(|m| m.borrow().category.to_string())
            .unwrap_or_else(|| "Special".to_string());
        let hint_str = format!("{} Shell Side Arm", category);
        battle.hint(&hint_str, false, None);
    }

    EventResult::Continue
}

/// onAfterSubDamage(damage, target, source, move) {
///     if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
/// }
pub fn on_after_sub_damage(
    battle: &mut Battle,
    _damage: i32,
    target_pos: (usize, usize),
    source_pos: (usize, usize),
    active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // Get active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
    let is_ally = battle.is_ally(source_pos, target_pos);

    if !is_ally {
        let category = active_move_ref.category.as_str();
        let hint_str = format!("{} Shell Side Arm", category);
        battle.hint(&hint_str, false, None);
    }

    EventResult::Continue
}
