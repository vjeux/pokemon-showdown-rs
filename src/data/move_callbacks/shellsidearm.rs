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
pub fn on_prepare_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    // if (!source.isAlly(target)) {
    //     this.attrLastMove('[anim] Shell Side Arm ' + move.category);
    // }
    let is_ally = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.is_ally(target)
    };

    if !is_ally {
        let category = battle.active_move.as_ref().map(|m| m.category.as_str()).unwrap_or("Special");
        battle.attr_last_move(&format!("[anim] Shell Side Arm {}", category));
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
    use crate::dex_data::ID;

    // if (!target) return;
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const atk = pokemon.getStat('atk', false, true);
    // const spa = pokemon.getStat('spa', false, true);
    // const def = target.getStat('def', false, true);
    // const spd = target.getStat('spd', false, true);
    let atk = battle.get_stat(pokemon_pos, "atk", false, true);
    let spa = battle.get_stat(pokemon_pos, "spa", false, true);
    let def = battle.get_stat(target_pos, "def", false, true);
    let spd = battle.get_stat(target_pos, "spd", false, true);

    // const physical = Math.floor(Math.floor(Math.floor(Math.floor(2 * pokemon.level / 5 + 2) * 90 * atk) / def) / 50);
    // const special = Math.floor(Math.floor(Math.floor(Math.floor(2 * pokemon.level / 5 + 2) * 90 * spa) / spd) / 50);
    let level = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.level
    };

    let physical = ((((2 * level / 5 + 2) * 90 * atk) / def) / 50) as i32;
    let special = ((((2 * level / 5 + 2) * 90 * spa) / spd) / 50) as i32;

    // if (physical > special || (physical === special && this.randomChance(1, 2))) {
    //     move.category = 'Physical';
    //     move.flags.contact = 1;
    // }
    if physical > special || (physical == special && battle.random_chance(1, 2)) {
        if let Some(active_move) = &mut battle.active_move {
            active_move.category = ID::from("Physical");
            active_move.flags.contact = true;
        }
    }

    EventResult::Continue
}

/// onHit(target, source, move) {
///     // Shell Side Arm normally reveals its category via animation on cart, but doesn't play either custom animation against allies
///     if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
/// }
pub fn on_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    // if (!source.isAlly(target)) this.hint(move.category + " Shell Side Arm");
    let is_ally = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.is_ally(target)
    };

    if !is_ally {
        let category = battle.active_move.as_ref().map(|m| m.category.as_str()).unwrap_or("Special");
        battle.hint(&format!("{} Shell Side Arm", category));
    }

    EventResult::Continue
}

