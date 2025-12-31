//! Electro Ball Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target) {
///     let ratio = Math.floor(pokemon.getStat('spe') / target.getStat('spe'));
///     if (!isFinite(ratio)) ratio = 0;
///     const bp = [40, 60, 80, 120, 150][Math.min(ratio, 4)];
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // let ratio = Math.floor(pokemon.getStat('spe') / target.getStat('spe'));
    let pokemon_speed = battle.get_pokemon_stat(pokemon, crate::dex_data::StatID::Spe, false, false);
    let target_speed = battle.get_pokemon_stat(target, crate::dex_data::StatID::Spe, false, false);

    // if (!isFinite(ratio)) ratio = 0;
    let ratio = if target_speed == 0 {
        0
    } else {
        pokemon_speed / target_speed
    };

    // const bp = [40, 60, 80, 120, 150][Math.min(ratio, 4)];
    let bp_table = [40, 60, 80, 120, 150];
    let index = std::cmp::min(ratio as usize, 4);
    let bp = bp_table[index];

    // this.debug(`BP: ${bp}`);
    battle.debug(&format!("BP: {}", bp));

    // return bp;
    EventResult::Number(bp)
}
