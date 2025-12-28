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
pub fn on_prepare_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // onPrepareHit(target, source, move) {
    //     if (!source.isAlly(target)) {
    //         this.attrLastMove('[anim] Shell Side Arm ' + move.category);
    //     }
    // }
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!source.isAlly(target)) {
    let is_ally = battle.is_ally(source, target);
    if !is_ally {
        // this.attrLastMove('[anim] Shell Side Arm ' + move.category);
        let category = {
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            active_move.category.clone()
        };

        let anim_string = format!("[anim] Shell Side Arm {}", category);
        battle.attr_last_move(&[&anim_string]);
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
    // onModifyMove(move, pokemon, target) {
    //     if (!target) return;
    //     const atk = pokemon.getStat('atk', false, true);
    //     const spa = pokemon.getStat('spa', false, true);
    //     const def = target.getStat('def', false, true);
    //     const spd = target.getStat('spd', false, true);
    //     const physical = Math.floor(Math.floor(Math.floor(Math.floor(2 * pokemon.level / 5 + 2) * 90 * atk) / def) / 50);
    //     const special = Math.floor(Math.floor(Math.floor(Math.floor(2 * pokemon.level / 5 + 2) * 90 * spa) / spd) / 50);
    //     if (physical > special || (physical === special && this.randomChance(1, 2))) {
    //         move.category = 'Physical';
    //         move.flags.contact = 1;
    //     }
    // }
    let pokemon = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const atk = pokemon.getStat('atk', false, true);
    // const spa = pokemon.getStat('spa', false, true);
    // const def = target.getStat('def', false, true);
    // const spd = target.getStat('spd', false, true);
    let atk = battle.get_stat(pokemon, "atk", false, true);
    let spa = battle.get_stat(pokemon, "spa", false, true);
    let def = battle.get_stat(target, "def", false, true);
    let spd = battle.get_stat(target, "spd", false, true);

    // const physical = Math.floor(Math.floor(Math.floor(Math.floor(2 * pokemon.level / 5 + 2) * 90 * atk) / def) / 50);
    // const special = Math.floor(Math.floor(Math.floor(Math.floor(2 * pokemon.level / 5 + 2) * 90 * spa) / spd) / 50);
    let level = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.level
    };

    let physical = {
        let step1 = 2 * level / 5 + 2;
        let step2 = step1 * 90 * atk;
        let step3 = step2 / def;
        step3 / 50
    };

    let special = {
        let step1 = 2 * level / 5 + 2;
        let step2 = step1 * 90 * spa;
        let step3 = step2 / spd;
        step3 / 50
    };

    // if (physical > special || (physical === special && this.randomChance(1, 2))) {
    //     move.category = 'Physical';
    //     move.flags.contact = 1;
    // }
    if physical > special || (physical == special && battle.random_chance(1, 2)) {
        let active_move = match &mut battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        active_move.category = "Physical".to_string();
        active_move.flags.contact = true;
    }

    EventResult::Continue
}

/// onHit(target, source, move) {
///     // Shell Side Arm normally reveals its category via animation on cart, but doesn't play either custom animation against allies
///     if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // onHit(target, source, move) {
    //     // Shell Side Arm normally reveals its category via animation on cart, but doesn't play either custom animation against allies
    //     if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
    // }
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
    let is_ally = battle.is_ally(source, target);
    if !is_ally {
        let category = {
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            active_move.category.clone()
        };

        let hint_string = format!("{} Shell Side Arm", category);
        battle.hint(&hint_string);
    }

    EventResult::Continue
}

/// onAfterSubDamage(damage, target, source, move) {
///     if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
/// }
pub fn on_after_sub_damage(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // onAfterSubDamage(damage, target, source, move) {
    //     if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
    // }
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
    let is_ally = battle.is_ally(source, target);
    if !is_ally {
        let category = {
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            active_move.category.clone()
        };

        let hint_string = format!("{} Shell Side Arm", category);
        battle.hint(&hint_string);
    }

    EventResult::Continue
}

