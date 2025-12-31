//! Gyro Ball Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target) {
///     let power = Math.floor(25 * target.getStat('spe') / pokemon.getStat('spe')) + 1;
///     if (!isFinite(power)) power = 1;
///     if (power > 150) power = 150;
///     this.debug(`BP: ${power}`);
///     return power;
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

    // let power = Math.floor(25 * target.getStat('spe') / pokemon.getStat('spe')) + 1;
    let target_spe = battle.get_pokemon_stat(target, crate::dex_data::StatID::Spe, false, false);
    let pokemon_spe = battle.get_pokemon_stat(pokemon, crate::dex_data::StatID::Spe, false, false);

    let mut power = if pokemon_spe == 0 {
        1
    } else {
        (25 * target_spe / pokemon_spe) + 1
    };

    // if (!isFinite(power)) power = 1;
    // Already handled by the division check above

    // if (power > 150) power = 150;
    if power > 150 {
        power = 150;
    }

    // this.debug(`BP: ${power}`);
    battle.debug(&format!("BP: {}", power));

    // return power;
    EventResult::Number(power)
}
