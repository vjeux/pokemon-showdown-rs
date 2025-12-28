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
pub fn base_power_callback(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => match battle.pokemon_at(pos.0, pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        },
        None => return EventResult::Continue,
    };

    // Calculate base power based on target's HP
    let hp = target.hp;
    let max_hp = target.maxhp;

    // Direct port of: Math.floor(Math.floor((120 * (100 * Math.floor(hp * 4096 / maxHP)) + 2048 - 1) / 4096) / 100) || 1
    let bp = if max_hp > 0 {
        let inner = (hp * 4096) / max_hp;
        let middle = (120 * (100 * inner) + 2048 - 1) / 4096;
        let result = middle / 100;
        if result == 0 { 1 } else { result }
    } else {
        1
    };

    // Note: JS has this.debug call which we don't have infrastructure for yet
    // this.debug(`BP for ${hp}/${maxHP} HP: ${bp}`);
    EventResult::Number(bp)
}
