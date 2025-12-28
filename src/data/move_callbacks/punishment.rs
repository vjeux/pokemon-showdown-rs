//! Punishment Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target) {
///     let power = 60 + 20 * target.positiveBoosts();
///     if (power > 200) power = 200;
///     this.debug(`BP: ${power}`);
///     return power;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // let power = 60 + 20 * target.positiveBoosts();
    let positive_boosts = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Count all positive boosts (atk, def, spa, spd, spe, accuracy, evasion)
        let mut count = 0i32;
        if target_pokemon.boosts.atk > 0 { count += target_pokemon.boosts.atk; }
        if target_pokemon.boosts.def > 0 { count += target_pokemon.boosts.def; }
        if target_pokemon.boosts.spa > 0 { count += target_pokemon.boosts.spa; }
        if target_pokemon.boosts.spd > 0 { count += target_pokemon.boosts.spd; }
        if target_pokemon.boosts.spe > 0 { count += target_pokemon.boosts.spe; }
        if target_pokemon.boosts.accuracy > 0 { count += target_pokemon.boosts.accuracy; }
        if target_pokemon.boosts.evasion > 0 { count += target_pokemon.boosts.evasion; }
        count
    };

    let mut power = 60 + 20 * positive_boosts;

    // if (power > 200) power = 200;
    if power > 200 {
        power = 200;
    }

    // this.debug(`BP: ${power}`);
    battle.debug(&format!("BP: {}", power));

    // return power;
    EventResult::Number(power)
}

