//! Wring Out Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     const hp = target.hp;
///     const maxHP = target.maxhp;
///     const bp = Math.floor(Math.floor((120 * (100 * Math.floor(hp * 4096 / maxHP)) + 2048 - 1) / 4096) / 100) || 1;
///     this.debug(`BP for ${hp}/${maxHP} HP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let hp = target.hp;
    let maxhp = target.maxhp;

    // Formula: floor(floor((120 * (100 * floor(hp * 4096 / maxhp)) + 2048 - 1) / 4096) / 100) || 1
    let bp = if maxhp > 0 {
        let ratio = (hp * 4096) / maxhp;
        let intermediate = (120 * (100 * ratio) + 2048 - 1) / 4096;
        let result = intermediate / 100;
        if result == 0 { 1 } else { result }
    } else {
        1
    };

    battle.debug(&format!("BP for {}/{} HP: {}", hp, maxhp, bp));
    EventResult::Number(bp)
}

