//! Heat Crash Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target) {
///     const targetWeight = target.getWeight();
///     const pokemonWeight = pokemon.getWeight();
///     let bp;
///     if (pokemonWeight >= targetWeight * 5) {
///         bp = 120;
///     } else if (pokemonWeight >= targetWeight * 4) {
///         bp = 100;
///     } else if (pokemonWeight >= targetWeight * 3) {
///         bp = 80;
///     } else if (pokemonWeight >= targetWeight * 2) {
///         bp = 60;
///     } else {
///         bp = 40;
///     }
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the pokemon (user of the move)
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get the target
    let target = match target_pos {
        Some(pos) => match battle.pokemon_at(pos.0, pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        },
        None => return EventResult::Continue,
    };

    // Get weights
    let pokemon_weight = pokemon.get_weight();
    let target_weight = target.get_weight();

    // Calculate base power based on weight ratio
    let bp = if pokemon_weight >= target_weight * 5 {
        120
    } else if pokemon_weight >= target_weight * 4 {
        100
    } else if pokemon_weight >= target_weight * 3 {
        80
    } else if pokemon_weight >= target_weight * 2 {
        60
    } else {
        40
    };

    // this.debug(`BP: ${bp}`);
    battle.debug(&format!("BP: {}", bp));

    EventResult::Number(bp)
}

/// onTryHit(target, pokemon, move) {
///     if (target.volatiles['dynamax']) {
///         this.add('-fail', pokemon, 'Dynamax');
///         this.attrLastMove('[still]');
///         return null;
///     }
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    // Get the target
    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // if (target.volatiles['dynamax'])
    if target.has_volatile(&ID::from("dynamax")) {
        // Get source ident for the add call
        let source_ident = {
            let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source.get_slot()
        };

        // this.add('-fail', pokemon, 'Dynamax');
        battle.add("-fail", &[source_ident.as_str().into(), "Dynamax".into()]);

        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // return null;
        return EventResult::Stop;
    }

    EventResult::Continue
}
