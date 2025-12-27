//! Crush Grip Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target) {
///     const hp = target.hp;
///     const maxHP = target.maxhp;
///     const bp = Math.floor(Math.floor((120 * (100 * Math.floor(hp * 4096 / maxHP)) + 2048 - 1) / 4096) / 100) || 1;
///     this.debug(`BP for ${hp}/${maxHP} HP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Get target
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
        (target_pokemon.hp as f64, target_pokemon.max_hp as f64)
    };

    // const bp = Math.floor(Math.floor((120 * (100 * Math.floor(hp * 4096 / maxHP)) + 2048 - 1) / 4096) / 100) || 1;
    let bp = if max_hp > 0.0 {
        let inner = (hp * 4096.0 / max_hp).floor();
        let middle = (120.0 * (100.0 * inner) + 2048.0 - 1.0) / 4096.0;
        let result = (middle.floor() / 100.0).floor();
        if result > 0.0 {
            result as i32
        } else {
            1
        }
    } else {
        1
    };

    // this.debug(`BP for ${hp}/${maxHP} HP: ${bp}`);
    battle.debug(&format!("BP for {}/{} HP: {}", hp, max_hp, bp));

    // return bp;
    EventResult::Int(bp)
}

