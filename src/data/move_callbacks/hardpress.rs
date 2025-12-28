//! Hard Press Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target) {
///     const hp = target.hp;
///     const maxHP = target.maxhp;
///     const bp = Math.floor(Math.floor((100 * (100 * Math.floor(hp * 4096 / maxHP)) + 2048 - 1) / 4096) / 100) || 1;
///     this.debug(`BP for ${hp}/${maxHP} HP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(battle: &mut Battle, _pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const hp = target.hp;
    // const maxHP = target.maxhp;
    let (hp, max_hp) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (target_pokemon.hp, target_pokemon.maxhp)
    };

    // const bp = Math.floor(Math.floor((100 * (100 * Math.floor(hp * 4096 / maxHP)) + 2048 - 1) / 4096) / 100) || 1;
    let bp = if max_hp == 0 {
        1
    } else {
        let inner_floor = (hp * 4096 / max_hp) as i32;
        let middle_calc = (100 * (100 * inner_floor) + 2048 - 1) / 4096;
        let outer_floor = middle_calc / 100;
        if outer_floor == 0 { 1 } else { outer_floor }
    };

    // this.debug(`BP for ${hp}/${maxHP} HP: ${bp}`);
    battle.debug(&format!("BP for {}/{} HP: {}", hp, max_hp, bp));

    // return bp;
    EventResult::Number(bp)
}

