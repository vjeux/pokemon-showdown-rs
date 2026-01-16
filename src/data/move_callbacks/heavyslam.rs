//! Heavy Slam Move
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
    use crate::event::EventTarget;

    // Get the target position
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get base weights - getWeight() runs ModifyWeight event
    // JavaScript: const pokemonWeight = pokemon.getWeight();
    // getWeight() { const weighthg = this.battle.runEvent('ModifyWeight', this, null, null, this.weighthg); return Math.max(1, weighthg); }
    let pokemon_base_weight = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.weight_hg
    };
    let pokemon_weight_result = battle.run_event(
        "ModifyWeight",
        Some(EventTarget::Pokemon(pokemon_pos)),
        None,
        None,
        EventResult::Number(pokemon_base_weight),
        false,
        false,
    );
    let pokemon_weight = match pokemon_weight_result {
        EventResult::Number(w) => w.max(1),
        _ => pokemon_base_weight.max(1),
    };

    // JavaScript: const targetWeight = target.getWeight();
    let target_base_weight = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.weight_hg
    };
    let target_weight_result = battle.run_event(
        "ModifyWeight",
        Some(EventTarget::Pokemon(target)),
        None,
        None,
        EventResult::Number(target_base_weight),
        false,
        false,
    );
    let target_weight = match target_weight_result {
        EventResult::Number(w) => w.max(1),
        _ => target_base_weight.max(1),
    };

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
    target_pos: (usize, usize),
    source_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    // JavaScript: onTryHit(target, pokemon, move) - target comes first, pokemon (source) second
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

        // return null; - prevents the move from executing
        return EventResult::Null;
    }

    EventResult::Continue
}
